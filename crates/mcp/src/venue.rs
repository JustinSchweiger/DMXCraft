//! Werkzeuge für den Veranstaltungsort (3D-Visualisierer).

use dmxcraft_core::venue::{Material, Object, Shape, Transform};
use dmxcraft_core::{Engine, FixtureId};
use serde_json::{json, Value};

type R = Result<Value, String>;

fn vec3(a: &Value, k: &str) -> Result<Option<[f32; 3]>, String> {
    match a.get(k) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(v)) if v.len() == 3 => {
            let mut r = [0.0; 3];
            for (o, x) in r.iter_mut().zip(v) {
                *o = x.as_f64().ok_or(format!("`{k}`: Zahlen erwartet"))? as f32;
            }
            Ok(Some(r))
        }
        _ => Err(format!("`{k}`: [x, y, z] erwartet")),
    }
}

fn num(a: &Value, k: &str) -> Option<f32> {
    a.get(k).and_then(Value::as_f64).map(|v| v as f32)
}

fn object_json(o: &Object, world: Option<[[f32; 4]; 4]>) -> Value {
    json!({
        "id": o.id,
        "name": o.name,
        "shape": o.shape,
        "position": o.transform.position,
        "rotation": o.transform.rotation,
        "scale": o.transform.scale,
        "world_position": world.map(|m| [m[3][0], m[3][1], m[3][2]]),
        "color": format!("#{:02x}{:02x}{:02x}", o.material.color[0], o.material.color[1], o.material.color[2]),
        "opacity": o.material.opacity,
        "locked": o.locked,
        "hidden": o.hidden,
        "children": o.children.iter().map(|c| c.id).collect::<Vec<_>>(),
    })
}

pub fn get_venue(e: &Engine) -> R {
    let show = e.show();
    let v = &show.venue;
    let objects: Vec<Value> = v
        .flatten()
        .into_iter()
        .map(|(o, m)| object_json(o, Some(m)))
        .collect();
    let fixtures: Vec<Value> = show
        .fixtures
        .iter()
        .map(|f| {
            let p = show.placement3d(f.id);
            json!({
                "id": f.id.0, "name": f.name,
                "position": p.position, "rotation": p.rotation, "direction": p.direction(),
                "automatic": f.place3d.is_none(),
            })
        })
        .collect();
    Ok(json!({
        "name": v.name,
        "size": v.size,
        "environment": v.environment,
        "top_level": v.objects.iter().map(|o| o.id).collect::<Vec<_>>(),
        "objects": objects,
        "fixtures": fixtures,
        "units": "Meter; y zeigt nach oben, x nach rechts, z zum Publikum; Drehung in Grad (X, dann Y, dann Z)",
    }))
}

pub fn set_venue(e: &mut Engine, a: &Value) -> R {
    let v = &mut e.show_mut().venue;
    if let Some(n) = a.get("name").and_then(Value::as_str) {
        v.name = n.to_string();
    }
    if let Some(s) = vec3(a, "size")? {
        v.size = s.map(|x| x.clamp(1.0, 200.0));
    }
    if let Some(x) = num(a, "ambient") {
        v.environment.ambient = x.clamp(0.0, 1.0);
    }
    if let Some(x) = num(a, "beams") {
        v.environment.beams = x.clamp(0.0, 1.0);
    }
    if let Some(x) = num(a, "fog") {
        v.environment.fog = x.clamp(0.0, 1.0);
    }
    Ok(json!({ "name": v.name, "size": v.size, "environment": v.environment }))
}

fn apply_object(o: &mut Object, a: &Value) -> Result<(), String> {
    if let Some(n) = a.get("name").and_then(Value::as_str) {
        o.name = n.to_string();
    }
    if let Some(s) = a.get("shape") {
        o.shape = serde_json::from_value(s.clone()).map_err(|e| format!("`shape`: {e}"))?;
    }
    if let Some(p) = vec3(a, "position")? {
        o.transform.position = p;
    }
    if let Some(r) = vec3(a, "rotation")? {
        o.transform.rotation = r;
    }
    if let Some(s) = vec3(a, "scale")? {
        o.transform.scale = s;
    }
    if let Some(c) = a.get("color").and_then(Value::as_str) {
        o.material.color = crate::tools::parse_hex(c)?;
    }
    if let Some(x) = num(a, "opacity") {
        o.material.opacity = x.clamp(0.05, 1.0);
    }
    if let Some(x) = num(a, "gloss") {
        o.material.gloss = x.clamp(0.0, 1.0);
    }
    if let Some(x) = num(a, "emissive") {
        o.material.emissive = x.clamp(0.0, 1.0);
    }
    if let Some(b) = a.get("locked").and_then(Value::as_bool) {
        o.locked = b;
    }
    if let Some(b) = a.get("hidden").and_then(Value::as_bool) {
        o.hidden = b;
    }
    Ok(())
}

pub fn add_object(e: &mut Engine, a: &Value) -> R {
    let shape: Shape = serde_json::from_value(a.get("shape").cloned().ok_or("`shape` fehlt")?)
        .map_err(|e| format!("`shape`: {e}"))?;
    let v = &mut e.show_mut().venue;
    let name = a.get("name").and_then(Value::as_str).unwrap_or("Objekt");
    let mut o = v.object(name, shape, Transform::default(), Material::default());
    apply_object(&mut o, a)?;
    let id = o.id;
    match a.get("parent").and_then(Value::as_u64) {
        Some(p) => {
            let parent = v.find_mut(p as u32).ok_or("Elternobjekt nicht gefunden")?;
            if !matches!(parent.shape, Shape::Group) {
                return Err("Elternobjekt muss eine Gruppe sein".into());
            }
            parent.children.push(o);
        }
        None => v.objects.push(o),
    }
    Ok(json!({ "id": id }))
}

pub fn update_object(e: &mut Engine, a: &Value) -> R {
    let id = a.get("id").and_then(Value::as_u64).ok_or("`id` fehlt")? as u32;
    let v = &mut e.show_mut().venue;
    let o = v.find_mut(id).ok_or("Objekt nicht gefunden")?;
    apply_object(o, a)?;
    let o = o.clone();
    Ok(object_json(&o, None))
}

pub fn delete_objects(e: &mut Engine, ids: &[u32]) -> R {
    let v = &mut e.show_mut().venue;
    let removed = ids.iter().filter(|id| v.remove(**id).is_some()).count();
    Ok(json!({ "removed": removed }))
}

pub fn place_fixtures(e: &mut Engine, a: &Value) -> R {
    let ids: Vec<FixtureId> = a
        .get("fixtures")
        .and_then(Value::as_array)
        .ok_or("`fixtures` fehlt")?
        .iter()
        .filter_map(Value::as_u64)
        .map(|v| FixtureId(v as u32))
        .collect();
    let position = vec3(a, "position")?;
    let spacing = vec3(a, "spacing")?.unwrap_or([0.0; 3]);
    let rotation = vec3(a, "rotation")?;
    let aim = vec3(a, "aim_at")?;
    let hang = a
        .get("hang_on_truss")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let auto = a.get("automatic").and_then(Value::as_bool).unwrap_or(false);
    let mut out = Vec::new();
    for (i, id) in ids.iter().enumerate() {
        let show = e.show();
        if show.fixture(*id).is_none() {
            return Err(format!("Gerät {} nicht gefunden", id.0));
        }
        if auto {
            if let Some(f) = e.show_mut().fixture_mut(*id) {
                f.place3d = None;
            }
            continue;
        }
        let mut p = show.placement3d(*id);
        if let Some(pos) = position {
            let k = i as f32;
            p.position = [
                pos[0] + spacing[0] * k,
                pos[1] + spacing[1] * k,
                pos[2] + spacing[2] * k,
            ];
        }
        if hang {
            if let Some(h) = show.venue.nearest_truss(p.position) {
                p.position = h.point;
            }
        }
        if let Some(r) = rotation {
            p.rotation = r;
        }
        if let Some(t) = aim {
            p.aim_at(t);
        }
        if let Some(f) = e.show_mut().fixture_mut(*id) {
            f.place3d = Some(p);
        }
        out.push(json!({ "id": id.0, "position": p.position, "rotation": p.rotation }));
    }
    Ok(json!({ "placed": out }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_truss_and_hang_fixture() {
        let mut e = Engine::new(dmxcraft_core::Show::default());
        let r = add_object(
            &mut e,
            &json!({ "name": "T1", "shape": { "type": "truss", "length": 3.0 }, "position": [0.0, 5.0, 0.0] }),
        )
        .unwrap();
        let id = r["id"].as_u64().unwrap() as u32;
        assert!(e.show().venue.find(id).is_some());
        update_object(&mut e, &json!({ "id": id, "color": "#ff0000" })).unwrap();
        assert_eq!(e.show().venue.find(id).unwrap().material.color, [255, 0, 0]);
        let v = get_venue(&e).unwrap();
        assert!(v["objects"]
            .as_array()
            .unwrap()
            .iter()
            .any(|o| o["id"] == id));
        delete_objects(&mut e, &[id]).unwrap();
        assert!(e.show().venue.find(id).is_none());
    }
}

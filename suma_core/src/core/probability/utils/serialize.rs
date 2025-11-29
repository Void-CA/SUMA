// Copia y pega esta función en el mismo archivo o en un módulo utils
use serde::{Serialize, Serializer};
use std::collections::HashMap;

fn serialize_complex_key<S, K, V>(
    map: &HashMap<K, V>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize,
    V: Serialize,
{
    use serde::ser::SerializeMap;

    let mut map_ser = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        // Convierte la llave (Vector) a String JSON: ["A", "B"] -> "[\"A\", \"B\"]"
        let key_string = serde_json::to_string(k).map_err(serde::ser::Error::custom)?;
        map_ser.serialize_entry(&key_string, v)?;
    }
    map_ser.end()
}
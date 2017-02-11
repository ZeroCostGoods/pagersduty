pub type Abilities = Vec<String>;

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_serde() {
        let mut file = File::open("testdata/types/abilities.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        // Verify deserialization.
        let abilities: Abilities = serde_json::from_str(&data).unwrap();
        let expected: Abilities = vec![
            "sso".into(),
            "advanced_reports".into(),
            "teams".into(),
            "read_only_users".into(),
            "team_responders".into(),
            "service_support_hours".into(),
            "urgencies".into(),
            "manage_schedules".into(),
            "manage_api_keys".into(),
            "coordinated_responding".into(),
            "using_alerts_on_any_service".into(),
            "event_rules".into(),
            "coordinated_responding_preview".into(),
            "preview_incident_alert_split".into(),
            "features_in_use_preventing_downgrade_to".into(),
            "feature_to_plan_map".into(),
        ];
        assert_eq!(abilities, expected);

        // Verify that serialization round-trips.
        let expected: serde_json::Value = serde_json::from_str(&data).unwrap();
        let serialized: serde_json::Value = serde_json::from_str(
            serde_json::to_string(&abilities).unwrap().as_ref()
        ).unwrap();
        assert_eq!(serialized, expected)
    }
}


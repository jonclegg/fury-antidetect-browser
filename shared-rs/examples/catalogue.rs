// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2026 Bogdan Shapovalov and the Fury authors

// Every catalogue persona with its core config, for a launcher that is not the
// agent (Cloakroom's, for one). The context is a placeholder: the caller
// replaces `locale`, `navigator.languages`, `noise` and `geolocation` per
// profile, which is why the seed here is 0 and the geolocation is left out.
//
//     cargo run -p fury-shared --example catalogue > personas.json

fn main() {
    let ctx = fury_shared::ProfileContext {
        timezone: "UTC".into(),
        languages: vec!["en-US".into(), "en".into()],
        ui_locale: "en-US".into(),
        geolocation: None,
        chrome_major: 155,
        chrome_full_version: "155.0.8059.12".into(),
    };
    let out: Vec<serde_json::Value> = fury_shared::catalogue::all()
        .into_iter()
        .map(|p| {
            p.validate().expect("catalogue persona is inconsistent");
            serde_json::json!({
                "id": p.id,
                "weight": p.weight,
                "os": p.os.name,
                "screen": {
                    "width": p.screen.width,
                    "height": p.screen.height,
                    "devicePixelRatio": p.screen.device_pixel_ratio,
                },
                "config": p.derive_core_config(0, &ctx),
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

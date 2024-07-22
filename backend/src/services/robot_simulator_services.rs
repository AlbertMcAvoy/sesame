use std::{thread::sleep, time::Duration};

use chrono::Utc;
use diesel::prelude::*;
use crate::{models::{database::DatabaseConnection, history::{Actions, NewHistory}, water_closet::WaterCloset}, schema::{histories::dsl::histories, water_closets::{dsl::water_closets, is_available, is_door_locked, is_door_opened}}};


pub fn scaning_opening_door(water_closet: WaterCloset, scan_mode: &str, conn: &mut DatabaseConnection) -> Result<String, String> {
    match register_scan_mode(&water_closet, scan_mode, conn) {
        Ok(_) => {
            let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::DoorOpening))
            .execute(conn);
            sleep(Duration::new(1, 0));
            let _ = diesel::update(water_closets.find(water_closet.id))
                .set((
                    is_available.eq(false),
                    is_door_opened.eq(true),
                ))
                .execute(conn);
            sleep(Duration::new(5, 0));
            let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::DoorClosing))
                .execute(conn);
            sleep(Duration::new(1, 0));
            let _ = diesel::update(water_closets.find(water_closet.id))
                .set((
                    is_door_opened.eq(false),
                    is_door_locked.eq(true)
                ))
                .execute(conn);
            let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::LockClosing))
                .execute(conn);
            Ok(String::from("ENTERED"))
        },
        Err(err) => Err(err)
    }
}

fn register_scan_mode(water_closet: &WaterCloset, scan_mode: &str, conn: &mut DatabaseConnection) -> Result<(), String> {
    match scan_mode {
        "QR_CODE" => {
            let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::QRCodeScan))
                .execute(conn);
            Ok(())
        },
        "NFC_CODE" => {
            let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::NFCScan))
                .execute(conn);
            Ok(())
        },
        _ => Err(String::from("ERROR|Unknown scan mode"))
    }
}

pub fn leaving_opening_door(water_closet: WaterCloset, conn: &mut DatabaseConnection) {
    let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::LockOpening))
        .execute(conn);
    let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::DoorOpening))
        .execute(conn);
    sleep(Duration::new(1, 0));
    let _ = diesel::update(water_closets.find(water_closet.id))
        .set((
            is_door_opened.eq(true),
            is_door_locked.eq(false)
        ))
        .execute(conn);
    sleep(Duration::new(5, 0));
    let _ = diesel::insert_into(histories).values(generate_history(&water_closet, Actions::DoorClosing))
        .execute(conn);
    sleep(Duration::new(1, 0));
    let _ = diesel::update(water_closets.find(water_closet.id))
        .set((
            is_available.eq(true),
            is_door_opened.eq(false),
        ))
        .execute(conn);
}

fn generate_history(water_closet: &WaterCloset, action: Actions) -> NewHistory {
    NewHistory {
        water_closet_id: water_closet.id,
        datetime: Utc::now().naive_utc(),
        action,
    }
}
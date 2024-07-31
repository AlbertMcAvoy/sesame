export interface WaterCloset {
    id: number,
    group_id: number,
    is_disabled: boolean,
    is_available: boolean,
    is_door_opened: boolean,
    is_door_locked: boolean,
    clean_state: CleanStates,
}

enum CleanStates {
    Cleaned,
    Used,
    Dirty,
    OutOfOrder,
}
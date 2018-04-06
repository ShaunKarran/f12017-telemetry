extern crate bincode;
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

use std::net::UdpSocket;

use bincode::{deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct UDPPacket {
    time: f32,
    lap_time: f32,
    lap_distance: f32,
    total_distance: f32,
    x: f32,  // World space position
    y: f32,  // World space position
    z: f32,  // World space position
    speed: f32,  // Speed of car in m/s
    xv: f32, // Velocity in world space
    yv: f32, // Velocity in world space
    zv: f32, // Velocity in world space
    xr: f32, // World space right direction
    yr: f32, // World space right direction
    zr: f32, // World space right direction
    xd: f32, // World space forward direction
    yd: f32, // World space forward direction
    zd: f32, // World space forward direction
    susp_pos: [f32; 4],    // Note: All wheel arrays have the order:
    susp_vel: [f32; 4],    // RL, RR, FL, FR
    wheel_speed: [f32; 4],
    throttle: f32,
    steer: f32,
    brake: f32,
    clutch: f32,
    gear: f32,
    gforce_lat: f32,
    gforce_lon: f32,
    lap: f32,
    engine_rate: f32,
    sli_pro_native_support: f32, // SLI Pro support
    car_position: f32,   // car race position
    kers_level: f32, // kers energy left
    kers_max_level: f32, // kers maximum energy
    drs: f32,    // 0 = off, 1 = on
    traction_control: f32,   // 0 (off) - 2 (high)
    anti_lock_brakes: f32,   // 0 (off) - 1 (on)
    fuel_in_tank: f32,   // current fuel mass
    fuel_capacity: f32,  // fuel capacity
    in_pits: f32,    // 0 = none, 1 = pitting, 2 = in pit area
    sector: f32, // 0 = sector1, 1 = sector2, 2 = sector3
    sector1_time: f32,   // time of sector1 (or 0)
    sector2_time: f32,   // time of sector2 (or 0)
    brakes_temp: [f32; 4], // brakes temperature (centigrade)
    tyres_pressure: [f32; 4],  // tyres pressure PSI
    teainfo: f32,  // team ID
    total_laps: f32, // total number of laps in this race
    track_size: f32, // track size meters
    last_lap_time: f32,  // last lap time
    max_rpm: f32,    // cars max RPM, at which point the rev limiter will kick in
    idle_rpm: f32,   // cars idle RPM
    max_gears: f32,  // maximum number of gears
    session_type: f32,    // 0 = unknown, 1 = practice, 2 = qualifying, 3 = race
    drs_allowed: f32, // 0 = not allowed, 1 = allowed, -1 = invalid / unknown
    track_number: f32,   // -1 for unknown, 0-21 for tracks
    vehicle_fia_flags: f32,    // -1 = invalid/unknown, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
    era: f32,                        // era, 2017 (modern) or 1980 (classic)
    engine_temperature: f32,     // engine temperature (centigrade)
    gforce_vert: f32,    // vertical g-force component
    ang_vel_x: f32,  // angular velocity x-component
    ang_vel_y: f32,  // angular velocity y-component
    ang_vel_z: f32,  // angular velocity z-component
    tyres_temperature: [u8; 4],   // tyres temperature (centigrade)
    tyres_wear: [u8; 4],  // tyre wear percentage
    tyre_compound: u8,  // compound of tyre – 0 = ultra soft, 1 = super soft, 2 = soft, 3 = medium, 4 = hard, 5 = inter, 6 = wet
    front_brake_bias: u8,         // front brake bias (percentage)
    fuel_mix: u8,                 // fuel mix - 0 = lean, 1 = standard, 2 = rich, 3 = max
    current_lap_nvalid: u8,      // current lap invalid - 0 = valid, 1 = invalid
    tyres_damage: [u8; 4],    // tyre damage (percentage)
    front_left_wing_damage: u8, // front left wing damage (percentage)
    front_right_wing_damage: u8,    // front right wing damage (percentage)
    rear_wing_damage: u8,   // rear wing damage (percentage)
    engine_damage: u8,  // engine damage (percentage)
    gear_box_damage: u8,    // gear box damage (percentage)
    exhaust_damage: u8, // exhaust damage (percentage)
    pit_limiter_status: u8, // pit limiter status – 0 = off, 1 = on
    pit_speed_limit: u8,    // pit speed limit in mph
    session_time_left: f32,  // NEW: time left in session in seconds
    rev_lights_percent: u8,  // NEW: rev lights indicator (percentage)
    is_spectating: u8,  // NEW: whether the player is spectating
    spectator_car_index: u8,  // NEW: index of the car being spectated

    // Car data
    num_cars: u8,               // number of cars in data
    player_car_index: u8,           // index of player's car in the array
    car_data: [CarUDPData; 20],   // data for all cars on track

    yaw: f32,  // NEW (v1.8)
    pitch: f32,  // NEW (v1.8)
    roll: f32,  // NEW (v1.8)
    x_local_velocity: f32,          // NEW (v1.8) Velocity in local space
    y_local_velocity: f32,          // NEW (v1.8) Velocity in local space
    z_local_velocity: f32,          // NEW (v1.8) Velocity in local space
    susp_acceleration: [f32; 4],   // NEW (v1.8) RL, RR, FL, FR
    ang_acc_x: f32,                 // NEW (v1.8) angular acceleration x-component
    ang_acc_y: f32,                 // NEW (v1.8) angular acceleration y-component
    ang_acc_z: f32,                 // NEW (v1.8) angular acceleration z-component
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct CarUDPData {
    world_position: [f32; 3], // world co-ordinates of vehicle
    last_lap_time: f32,
    current_lap_time: f32,
    best_lap_time: f32,
    sector1_time: f32,
    sector2_time: f32,
    lap_distance: f32,
    driver_id: u8,
    team_id: u8,
    car_position: u8,     // UPDATED: track positions of vehicle
    current_lap_num: u8,
    tyre_compound: u8,   // compound of tyre – 0 = ultra soft, 1 = super soft, 2 = soft, 3 = medium, 4 = hard, 5 = inter, 6 = wet
    in_pits: u8,           // 0 = none, 1 = pitting, 2 = in pit area
    sector: u8,           // 0 = sector1, 1 = sector2, 2 = sector3
    current_lap_invalid: u8, // current lap invalid - 0 = valid, 1 = invalid
    penalties: u8,  // NEW: accumulated time penalties in seconds to be added
}

fn main() {
    let address = "192.168.0.5:20777";
    let socket = UdpSocket::bind(address).expect(&format!("Could not bind to address: {}", address));

    let mut input_buffer: [u8; 1289] = [0; 1289]; // The packet will always be 1289 bytes.

    loop {
        // read from the socket
        let _ = socket.recv_from(&mut input_buffer).unwrap();

        let packet: UDPPacket = deserialize(&input_buffer).unwrap();
        // let packet: UDPPacket = serde::from_bytes(&input_buffer).unwrap();

        // println!("\nBytes read: {}\nSource Address: {}", number_of_bytes, src_addr);
        // println!("\nPacket data:\n{:?}", packet);
        println!("Time: {}", packet.time);
        println!("Speed (KPH): {}", ms_to_kmh(packet.speed));
        // println!("Wheel Speeds: {:?}", packet.wheel_speed);
        println!("RPM: {}", packet.engine_rate);
        println!("Throttle: {}", packet.throttle);
        println!("Brake: {}", packet.brake);
        println!("Steer: {}", packet.steer);
        println!("Gear: {}", packet.gear);
        println!("Fuel Mix: {}", packet.fuel_mix);
        println!("Lap: {}", packet.lap);
    }
}

fn ms_to_kmh(speed: f32) -> f32 {
    speed * 3.6
}

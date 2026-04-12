// =====================================================================
// HAVERSINE FORMULA
// =====================================================================
// Do GPS points ke beech ka distance calculate karta hai
// Earth ki curvature ko bhi consider karta hai — isliye accurate hai
//
// Example:
//   Mumbai (19.076, 72.877) se Thane (19.197, 72.971) = ~15 km

const EARTH_RADIUS_KM: f64 = 6371.0;

/// Do lat/lon points ke beech distance kilometers mein
pub fn distance_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_r = lat1.to_radians();
    let lat2_r = lat2.to_radians();
    let dlat   = (lat2 - lat1).to_radians();
    let dlon   = (lon2 - lon1).to_radians();

    // Haversine formula
    let a = (dlat / 2.0).sin().powi(2)
          + lat1_r.cos() * lat2_r.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().asin();

    EARTH_RADIUS_KM * c
}

/// Kya dono users 5 km ke andar hain?
pub fn within_radius(lat1: f64, lon1: f64, lat2: f64, lon2: f64, radius_km: f64) -> bool {
    distance_km(lat1, lon1, lat2, lon2) <= radius_km
}

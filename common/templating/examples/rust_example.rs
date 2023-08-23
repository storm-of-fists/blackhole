fn main() {
    {% for accel_axis_name in imu.accelerometer.axes %}
    println!("{{ accel_axis_name }} is cool!");
    {% endfor %}
}

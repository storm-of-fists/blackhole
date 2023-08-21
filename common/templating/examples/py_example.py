class AccelerometerAxis:
    def __init__(self, name, calibration):
        self.name = name
        self.calibration = calibration


list_of_accels = [
    {% for accel_axis_name, accel_axis_info in imu.accelerometer.axes.items() %}
    AccelerometerAxis("{{ accel_axis_name }}", {{ accel_axis_info.calibration }}),
    {% endfor %}
]


for accel in list_of_accels:
    print(f"{accel.name} is cool!")
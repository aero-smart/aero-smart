with open('pid-feb-11-00-24.txt', 'r') as f:
    pid_data = f.read()

raw_airspeeds = []
filtered_airspeeds = []
feedforward_control = []
feedback_correction = []
throttles = []
duty_cycles = []
states = []

for line in pid_data.splitlines():
    time = line.split(' ')[0]
    if 'Airspeed Control | Current Airspeed: ' in line:
        # e.g. 8.595512 [INFO ] Airspeed Control | Current Airspeed: 8.690301 | Filtered Airspeed: 8.623456 | Setpoint: 9.0 (aerosmart_firmware src/algorithms/airspeed.rs:108)
        parts = line.split(' | ')
        current_airspeed = float(parts[1].split(': ')[1])
        filtered_airspeed = float(parts[2].split(': ')[1])
        setpoint = float(parts[3].split(': ')[1].split(' ')[0])
        raw_airspeeds.append((time, current_airspeed))
        filtered_airspeeds.append((time, filtered_airspeed))
    elif 'Airspeed Control | Feedforward Throttle: ' in line:
        # e.g. 8.595698 [INFO ] Airspeed Control | Feedforward Throttle: 409 | Current Airspeed: 8.690301 | Setpoint: 9.0 (aerosmart_firmware src/algorithms/airspeed.rs:119)
        parts = line.split(' | ')
        current_airspeed = float(parts[2].split(': ')[1])
        feedforward = float(parts[1].split(': ')[1])
        setpoint = float(parts[3].split(': ')[1].split(' ')[0])
        raw_airspeeds.append((time, current_airspeed))
        feedforward_control.append((time, feedforward))
    elif 'EDF Control | Measured Airspeed: ' in line:
        # 8.645836 [INFO ] EDF Control | Measured Airspeed: 2.227326 m/s | Setpoint: 9.0 m/s (aerosmart_firmware src/tasks/edf.rs:29)
        parts = line.split(' | ')
        measured_airspeed = float(parts[1].split(': ')[1].split(' ')[0])
        setpoint = float(parts[2].split(': ')[1].split(' ')[0])
        raw_airspeeds.append((time, measured_airspeed))
    elif 'Airspeed Control | Feedback Correction: ' in line:
        # 8.646149 [INFO ] Airspeed Control | Feedback Correction: 315.57562 | Reaching State: Reaching (aerosmart_firmware src/algorithms/airspeed.rs:126)
        parts = line.split(' | ')
        feedback = float(parts[1].split(': ')[1])
        state = parts[2].split(': ')[1]
        feedback_correction.append((time, feedback))
        states.append((time, state))
    elif 'Setting throttle compatible: DShot' in line:
        # 8.646285 [INFO ] Setting throttle compatible: DShot 409 -> Duty Cycle 3764 (aerosmart_firmware src/executors/edf_pwm.rs:49)
        parts = line.split(' -> Duty Cycle ')
        duty_cycle = int(parts[0].split(' ')[-1])
        duty_cycles.append((time, duty_cycle))
    elif 'Calculated Airspeed: ' in line:
        # 8.746354 [INFO ] Calculated Airspeed: 7.372464 m/s (aerosmart_firmware src/tasks/airspeed.rs:34)
        parts = line.split(': ')
        calculated_airspeed = float(parts[1].split(' ')[0])
        raw_airspeeds.append((time, calculated_airspeed))

import matplotlib.pyplot as plt
import numpy as np

raw_speeds_plot = (np.array([v for t, v in raw_airspeeds]), np.array([t for t, v in raw_airspeeds]))
filtered_speeds_plot = (np.array([v for t, v in filtered_airspeeds]), np.array([t for t, v in filtered_airspeeds]))
feedforward_plot = (np.array([v for t, v in feedforward_control]), np.array([t for t, v in feedforward_control]))
feedback_plot = (np.array([v for t, v in feedback_correction]), np.array([t for t, v in feedback_correction]))
duty_cycle_plot = (np.array([v for t, v in duty_cycles]), np.array([t for t, v in duty_cycles]))

print(raw_speeds_plot, filtered_speeds_plot)

plt.figure(figsize=(12, 8))

plt.plot(duty_cycle_plot[1], duty_cycle_plot[0], label='Duty Cycle', color='cyan')
# plt.axhline(y=0, color='r', linestyle='--', label='Zero Line')
plt.title('Duty Cycle Over Time')
plt.ylabel('Duty Cycle')
plt.legend()
plt.show()

plt.figure(figsize=(12, 8))

# plt.plot(raw_speeds_plot[1], raw_speeds_plot[0], label='Raw Airspeed', color='blue')
plt.plot(filtered_speeds_plot[1], filtered_speeds_plot[0], label='Filtered Airspeed', color='orange')
plt.axhline(y=setpoint, color='r', linestyle='--', label='Setpoint')
plt.title('Airspeed Control Analysis')
plt.ylabel('Airspeed (m/s)')
plt.show()

print(feedforward_plot, feedback_plot)

plt.figure(figsize=(12, 8))
plt.plot(feedforward_plot[1], feedforward_plot[0], label='Feedforward Throttle', color='green')
plt.plot(feedback_plot[1], feedback_plot[0], label='Feedback Correction', color='purple')
plt.plot(duty_cycle_plot[1], duty_cycle_plot[0], label='Duty Cycle', color='cyan')
plt.axhline(y=0, color='r', linestyle='--', label='Zero Line')
plt.title('Control Outputs')
plt.ylabel('Throttle Output')
plt.show()
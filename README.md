# 🚦 Traffic Simulation 🚗

Welcome to the Traffic Simulation project! 🌟 This simulation models a bustling intersection with two roads crossing each other, each featuring one lane in each direction. The goal? Keep vehicles moving smoothly and avoid collisions at all costs!

## 🌍 Environment and Rules

### 🛣️ Roads

```
                        North
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
     _______________|     |     |_______________
     ← ←                                     ← ←
East ---------------             --------------- West
     → →                                     → →
     _______________             _______________
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |     |     |
                    |  ↓  |  ↑  |
                    |  ↓  |  ↑  |
                        South
```

- The simulation features two roads forming a crossroads.
- Each road has one lane in each direction.
- Vehicles choose their path by turning left, right, or going straight through the intersection.

### 🚥 Traffic Lights

- Traffic lights control traffic flow at each lane's entrance to the intersection.
- Lights cycle between red and green.
- Design your own algorithm to minimize congestion (max 8 vehicles) and prevent collisions.

### 🚗 Vehicles

- Vehicles follow specific rules:
  - Each vehicle is color-coded based on its intended route.
  - Fixed velocity and safe following distance are maintained.
  - Vehicles stop at red lights and proceed on green.
  - Routes cannot be changed mid-simulation.
  - No special priority is given to emergency vehicles.

## ⌨️ Commands

- Use these keyboard commands to interact with the simulation:
  - Arrow Keys:
    - ↑ (Up): Spawn a vehicle heading south.
    - ↓ (Down): Spawn a vehicle heading north.
    - → (Right): Spawn a vehicle heading west.
    - ← (Left): Spawn a vehicle heading east.
  - r: Spawn a vehicle in a random direction.
  - Esc (Escape): End the simulation.

- Vehicles spawn with adequate spacing to prevent collisions.

## 🏁 Running the Simulation

- Start the simulation and observe traffic dynamics at the intersection.
- Use arrow keys to introduce vehicles and witness their interactions.
- Manage traffic lights to optimize vehicle flow.
- Press Esc to gracefully conclude the simulation.

## Author

- Author: Anass EL Karoumi [@Anasskm](https://github.com/Anasskm)
- Email: karoumi14@gmail.com




<br>
<p align="center"><i>Enjoy managing your virtual intersection and keeping traffic flowing smoothly! 🚦🚗</i></p>

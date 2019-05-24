# PONG
A Rust implementation of classic pong using [Piston](https://www.piston.rs/) and [nphysics](https://nphysics.org/). This is primarily being used as an intro for me to Rust and writing games. 

I have attempted to "glue" together the Piston window and nphysics world for this game. It feels a little fragile right now but I'm hoping that with more experience with the two crates I'll be able to harden it.

## Implementation

### Game
The world consists of the physics world, the ball, and the two players. When the world is initialized, it builds the physics world and creates colliders for the ball and the two players, which are the put into the world. Over every loop the game "updates", which steps the physics world once; this is when the physics engine does its work (I think). 

After updating the world, we render it. The game's call to render then renders both the ball and the players in the piston window.

### PongPlayer
The body essentially has an infinite mass via the nphysics `BodyStatus::Kinematic` so that when the ball collides it doesn't move. It should only be moved via player input.

### PongBall
The pong ball creates a custom material type in nphysics so that it does not lose a lot of inertia when it collides with players. The thought is to have all of its energy conserved every time it bounces. I'm not sure if this is the best way to accomplish this yet. 

---
### TODO
- [x] Player input
    * specifically the player inputs need to be async b/c they're currently blocking
    - [ ] Player input now uses a HashSet to find the keys that are currently being pressed. This feels very hacky and gross. It looks like there is a set of event traits for buttons that I may be able to leverage for individual buttons in the [piston input](https://github.com/PistonDevelopers/piston/blob/V0.33.0/src/input/src/button.rs#L32) module.
- [x] Scoring
    * Still need to show the score after a goal, and need to figure out when a player has won
- [x] Boundaries
- [x] Split the structs into their own files/modules
- [ ] The Game knows too much about how to create the ball and player objects. Should split that logic out into the appropriate mods.
- [ ] Need a menu to exit

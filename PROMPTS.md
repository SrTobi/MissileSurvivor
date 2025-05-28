Prompts used to vibe this
====

```
Hey. I have a little graphical project here, that draws a circle and a line, which turns by 90° per second. The line is red at the moment and has a radius of 300.
Please make it blue and reduce the radius to 200
```

```
Ok next task is to make the game end when I press ESC
```

```
Now I want to program the classic game Missile Commander in Rust using the already setup library macroquad.
I already added a basic initial app that sets up the view and draws a circle and a line that rotates.
Feel free to remove any code that I wrote, But I think the camera I create is already a good thing.
It basically configures the graphics in a way that you can savely draw into a canvas that has tha logical size of 800 by 600 and the center of that is in the center of the screen.

To remind you:
Missile commander is a game where Missiles (represented by just white lines) come from the top of the view area and fly to one of three bunkers that are on the bottom of the view area.
The player can click on the screen to shoot a missile of his own. The missile flies from the closest available bunker to the clicked position and explodes in a radius.
Missiles (regardless of whether they are enemy or the player's) that are whithin an explosion also explode with an explosion radius.
Every bunker can only fire a single missile at the same time.
If an enemy missile hits one of the bunkers that bunker is destroyed. The player looses if all bunkers are destroyed.

For now, there needs to be no progression in the game, so the rate of enemy missiles can stay the same and the player cannot upgrade anything. We will add that later.
The missiles and explosions should be white. The bunkers should sit on top of a small ground on the bottom of the screen... ground and bunkers should be yellow
```

```
Nice! ok. It seems we need to reuse the viewport better. Instead of creating it in the render method, let's create it in the update method and store it in the Game struct to be reused everywhere.
```

```
now we create a view_rewt, etc twice in the code. extract both usages into a function
```

```
I think create_viewport doesn't need any parameters. That will make the usage more easy. just move the view_rect into the function
```

```
Make it so that I can press any key to restart the game after a gameover
```

```
Extract missile explosion into a helper method
```

```
instead of checking whether a missile hit a bunker, store the target buker as an index and update it accordingly. Also store the missile direction in the missile struct
```

```
Unify EnemyMissle and PlayerMissle. make target_bunker_idx an optional
```

```
Please also use more helper methods like Missile::new and Binker::new
```

```
Split the game into multiple smaller files
```

```
Ok. Now I want to add some progression. First there should be a Timer in the top right corner that shows the time that has elapsed since the start of the game.
The timer should have fromat mm:ss. Depending on the time spend, enemy missiles should get faster and more frequent.
In the beginning they should be really slow. Player missiles should keep the speed they currently have. We'll add progression to that later
```

```
Please make the speed of a missile a field inside the Missile struct. also move stuff like get_enemy_missile_speed into constants.rs
```

```
the speed should be a parameter of Missile::new
```

```
move get_enemy_missile_spawn_interval to constants.rs. Also instead of time_since_last_spawn use time_until_next_missile_spawn. And randomize the spawn time from interval to interval. i.e. get the spawninterval from the get_spawn_interval method and then randomize it by 50% more or less
```

```
Don't use Frame::game_t for counting the game time. Use custom field in Game
```

```
Next I want that explosions to have two phases. In the first phase the explosion expands in a linear fashion like it does right now. After 80% of the way phase two starts and they only expand with decreasing speed, so that the explosion comes to a smooth stand still. both phases should take about the same time and it should never expand faster than the constant rate it had in the first phase
```

```
Move the growing mechanics of the explosion to explosion.rs. Also use an enum for Explosion::phase instead of a u8
```

```
add a method to check whether the explosion has ended and use it in update_explosions
```

```
Add a third phase where the explosion stays at maximum radius for a short time
```

```
I want to add some progression to the game for the player now. Actually leveling up will be implemented later. First create struct called Player. This struct should hold the levels of the skills of the player and functions to access the effect of that skill. Every skill starts at level 0. The skills are:
- Explosion Speed. Explosions caused by missiles from the player grow 20% faster per level
- Explosion After Glow. the static_durations of explosions caused by the player's missiles grows by 20% per level.
- Explosion Radius. The max radius of explosions caused by missiles from the player is 20% bigger per level
- Missile speed. Player missiles are 20% faster per level.
```

```
To hand the explosion effects add a struct ExplosionParams and impl Default and From<Player>. The Explosion::new should then take that struct and the pos vector
```

```
Ok. Next the leveling up mechanics. There should be an experience counter. Every time a missile is destroyed the experience counter fills with experience.  The experience counter is visualized by a blue line filling the top of the view from left to right. If the experience counter is full a blue star appears somewhere in the view (but above the bunkers). The higher missiles are destroyed, the more experience the player gets. At the top of the screen he gets 100 points. at the bunker level he gets 0. Interpolate in between. The first star requires 50 experience and every subsequent star requires 10% more experience. The stars don't do anything for now. We'll implement that later
```

```
You misunderstood me. The player does not level up. if the experience bar is full a Star is created at a random position. The player can shoot that star with an explosion and then he levels up a level. let's implement that.
```

```
Ok! time to level up. As long as level_ups_left is greater 0 show a skill selection menu. Choose 2 skills at random and let the player select one of them to level up. Create an enum with all skills and use that to write nice level up code.
```

```
Ok, nice, nice. But please replace the skill selection from keyboard with mouse
```

```
The two selection boxes are overflowing the selection panel. move them a little bit closer together
```

```
The bunkers are currently rectangles. can you make them have slopes on each side
```

```
Show the player's level under the timer
```

```
Rename the project from MCom/MissileCommander to MissileSurvivor
```
## Features

### Minions gathering resources

Minions should be spawned to gether resources

When minions are spawned in the base, they should locate resource, gather them and carry to gathering post

Dependency: [Building structures](#building-structures)

![Minions gathering resources mockup](./images/image1.png)

### Portal to fighting instance

Player should be able to enter portal and access instance where he can fight enemies

![Portalling to fighting arena mockup](./images/image2.png)

### Minion combat system

Player should be able to spawn minions that will fight for player

Initialy:
- Guardian minions (tank) - guards player
- Offence minions (dps) - attacks enemies

Dependencies: [Enemy combat system](#enemy-combat-system)

![Minions fighting mockup](./images/image3.png)

### Enemy combat system

Enemies should attack player.

Enemies should first target tanks (player defence), after that, the player

Enemy types:
- Melee (close range)
- Ranged (projectile attacks)

Dependencies: [Minion combat system](#minion-combat-system)

### Building structures

Player should be able to use gathered resources to build structures that would work as powerups and just let progress the game

Initialy:
- Gathering post (to gather resources)
- Minion *asembly?* (to create minions)
- Powerup *totem?* (to grant powerups for player)

### Player combat system

Player should be able to cast abilities that would either buff minions or debuff enemies

- Buff minion damage
- Heal minions
- Debuff enemy damage

Dependencies: [NPC combat system](#npc-combat-system)

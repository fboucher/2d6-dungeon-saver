
This is a text screensaver based on the the game 2D6 dungeon. It's supposed to run automatically. And generating a dungeon map as the explorer open doors, exploring the dungeon.

- Official Game Rule: ./docs/2d6 Rules.md
- Official Game flow: ./docs/2d6-flow-page1.png, ./docs/2d6-flow-page2.png
- Technical design: ./docs/Rules.md

Issues:
- [ ] When The explorer opened a door. The wall between the current room and the room where the explorer Enter. Should be adjoining wall. But currently it's not. The wall is duplicate. And then the explorer gets stuck. Into the new room. 
- [ ] When an explorer opened a door Yeah. Teleported in the middle of the next room. It shouldn't be the case. You go you should Walk through the door. 
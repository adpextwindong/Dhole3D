# TODO

https://www.scratchapixel.com/

https://www.youtube.com/watch?v=8CMh-D4de1I

Raycast debugging utility window

http://lodev.org/cgtutor/raycasting.html
http://www.playfuljs.com/a-first-person-engine-in-265-lines/
https://news.ycombinator.com/item?id=7842037
https://www.giantbomb.com/ray-casting/3015-1517/games/


// NOTES
// CURRENT : TODO gamestate -> pixel array (everything rendered to the window context texture)
//           TODO Get raycaster to work on a collumn level then 2D stage
//                Once simple colors are handled we should move to each wall having bitmap surfaces





// Asset handling https://rust-sdl2.github.io/rust-sdl2/sdl2/image/index.html
// ?Depth buffer
// TODO Move graphics code to renderer module
// TODO Add headbob (I guess their has to be a basic player height) to make moving around seem real
// TODO_FAR Gamestate handler, actual game once initial graphics are up

// TODO work on documentation of how the engine works so its easy to come back to
// world is 2d matrix of walls for now
// world -> pixel_array

// #Nice things
// TODO tileset editor to create json verson of maps
// TODO serialize maps to json
// TODO load maps from json via file or text entry box

// TODO CHORE: Find some way to edit the tileset easier
//TODO_FAR Move this to a seperate renderer file that takes the world ref
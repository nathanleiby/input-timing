- for each frame with input
  - display frame time
  - display input that happened in that frame
- capture midi input
  - display midi time

(1) GOAL: Show the offset of a midihit before or after the corresponding frame.
e.g.
"the last game loop frame was 25ms ago"
"your midi hit(s) occured 7ms BEFORE the most recent frame"

Then visualize this simply with rectangles and lines.

(2) Extension: consider if we can handle multiple midi hits in a frame?!
This might not be possible with how Macroquad does key presses(!) since it maxes at one per frame, I think

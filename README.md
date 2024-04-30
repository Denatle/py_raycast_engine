# py_raycast_engine
![output example](/output.gif)\
Example of usage:

```py
import raycast
from PIL import Image, ImageDraw

SIZE = 500

if __name__ == "__main__":
# 6.282 - 360
# 1.5705 - 90

    Map = [
        0b11111111111111111111111111111111,
        0b10000000000000000000000000000001,
        0b10100010010000000000000010000001,
        0b10000000000000000000000000000001,
        0b10000000001001000000000000000001,
        0b10001000000000000000000001000001,
        0b10000000000110100000000000000001,
        0b10010000000000000000000000000001,
        0b10000000000000000000000100000001,
        0b10010000010000000000000000000001,
        0b10000000000100000000100000000001,
        0b10000000000000000000000100000001,
        0b10000100000000010000000100000001,
        0b10000000100000000000000000000001,
        0b10000000000000000000000100000001,
        0b10000001000000001111000000000001,
        0b10010000000000000000000000000001,
        0b10010000000000000000000100000001,
        0b10001110001000010000000000000001,
        0b10000000000000000000100000001001,
        0b10000000000000100000000000010001,
        0b10000000010000000000000000100001,
        0b10000000000000010000000001000001,
        0b10000000000000000000000000000001,
        0b10000000100000000000000011000001,
        0b10000010000000000000100000000001,
        0b10000000000000000000000000000001,
        0b10000000010000000000000000000001,
        0b10000000000000001000000000000001,
        0b10000100000000000000000000000001,
        0b10000000000000000000000000000001,
        0b11111111111111111111111111111111,
    ]

    settings_state = raycast.create_settings_state(size=SIZE)

    game_state = raycast.create_game_state(Map, 16, 16, 0)
    for i in range(360):
        game_state = raycast.edit_game_state(game_state, 16, 16, i / 360 * 6.282)
        data = raycast.get_view(game_state, settings_state)
        img = Image.new("L", (SIZE, SIZE))
        draw = ImageDraw.Draw(img)
        draw.rectangle((0, 0, 500, 250), fill=(22,))
        draw.rectangle((0, 250, 500, 500), fill=(33,))
        for line in data:
            fill = int(line[2] / (SIZE / 2) * 255)
            draw.line([(line[0], line[1]), (line[0], line[1] + line[2] - 1)],
                      fill=(fill,),
                      width=1)
        img.save(f"img{i}.png")
```
## ToDo

 - [x] Raycast on rust
 - [x] Full settings setter in python
 - [x] Better structure
 - [x] Make it more friendly to python
 - [x] Make it work
 - [ ] Make description
 - [ ] Docs
 - [x] Publish repository

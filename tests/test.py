import raycast
from PIL import Image, ImageDraw

size = 500

if __name__ == "__main__":
    for i in range(625):
        # noinspection PyUnresolvedReferences
        raycast.set_state(8, 8, i/100)
        # noinspection PyUnresolvedReferences
        data = raycast.return_view()
        img = Image.new("L", (size, size))
        draw = ImageDraw.Draw(img)
        for line in data:
            draw.line([(line[0], line[1]), (line[0], line[1] + line[2] - 1)], fill=(int(line[2]+20),), width=1)
        # img.show()

        img.save(f"img{i}.png")

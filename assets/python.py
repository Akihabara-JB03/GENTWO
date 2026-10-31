
import resvg_py
from PIL import Image
from io import BytesIO
from pathlib import Path

def svg_to_ico(svg_path, ico_path):
    # SVGをPNGにレンダリング
    png_data = resvg_py.svg_to_bytes(
        svg_path=str(svg_path),
        width=256,
        height=256
    )

    # PNGを読み込む
    image = Image.open(BytesIO(png_data)).convert("RGBA")

    # ICOとして保存
    image.save(
        ico_path,
        format="ICO",
        sizes=[
            (16, 16),
            (24, 24),
            (32, 32),
            (48, 48),
            (64, 64),
            (128, 128),
            (256, 256)
        ]
    )

if __name__ == "__main__":
    svg_to_ico("gentwo_ico.svg", "gentwo.ico")
    print("Conversion completed!")

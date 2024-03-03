import argparse
import os
import win32ui
import win32gui
import win32con
import win32api
from PIL import Image

def extract_icon_from_exe(icon_in_path, icon_name, icon_out_path, out_width=24, out_height=24):
    ico_x = win32api.GetSystemMetrics(win32con.SM_CXICON)
    ico_y = win32api.GetSystemMetrics(win32con.SM_CYICON)

    large, small = win32gui.ExtractIconEx(icon_in_path, 0)
    win32gui.DestroyIcon(small[0])

    hdc = win32ui.CreateDCFromHandle(win32gui.GetDC(0))
    hbmp = win32ui.CreateBitmap()
    hbmp.CreateCompatibleBitmap(hdc, ico_x, ico_x)
    hdc = hdc.CreateCompatibleDC()

    hdc.SelectObject(hbmp)
    hdc.DrawIcon((0, 0), large[0])

    bmpstr = hbmp.GetBitmapBits(True)
    icon = Image.frombuffer(
        'RGBA',
        (32, 32),
        bmpstr, 'raw', 'BGRA', 0, 1
    )

    full_outpath = os.path.join(icon_out_path, "{}.png".format(icon_name))
    icon_resized = icon.resize((out_width, out_height))
    icon_resized.save(full_outpath)
    # return the final path to the image
    return full_outpath

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Extract icon from an exe file")
    parser.add_argument("icon_in_path", help="Input path to the exe file")
    parser.add_argument("icon_name", help="Name of the icon")
    parser.add_argument("icon_out_path", help="Output path for the extracted icon")
    parser.add_argument("--out_width", type=int, default=56, help="Width of the output icon (default: 56)")
    parser.add_argument("--out_height", type=int, default=56, help="Height of the output icon (default: 56)")
    args = parser.parse_args()

    extracted_icon_path = extract_icon_from_exe(args.icon_in_path, args.icon_name, args.icon_out_path, args.out_width, args.out_height)
    print("Icon extracted and saved at:", extracted_icon_path)

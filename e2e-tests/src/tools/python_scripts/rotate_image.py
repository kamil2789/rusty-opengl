from PIL import Image


image = Image.open("template.png")
image = image.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
image.show()
image.save("template.png")
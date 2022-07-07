import hashlib
import sys
from PIL import Image
import os


def get_path_to_templates():
    dir = os.path.basename(os.getcwd())
    if dir == "e2e-tests":
        return "image_templates/"
    else:
        return "e2e-tests/image_templates/"


TEMPLATE_IMAGE_NAME = sys.argv[1]
TEMPLATE_IMAGE_PATH = get_path_to_templates() + TEMPLATE_IMAGE_NAME
GENERATED_IMAGE_PATH = "test_result_" + TEMPLATE_IMAGE_NAME


def transform_vertically(path: str):
    image = Image.open(path)
    image = image.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
    image.save(path)


def get_hash(path: str):
    with open(path, "rb") as f:
        bytes = f.read()
        return hashlib.sha256(bytes).hexdigest()


def test_images():
    template_image_hash = get_hash(TEMPLATE_IMAGE_PATH)
    generated_image_hash = get_hash(GENERATED_IMAGE_PATH)

    if template_image_hash != generated_image_hash:
        sys.stderr.write(f"Images are not equal! {TEMPLATE_IMAGE_NAME}")
        sys.exit(1)


transform_vertically(GENERATED_IMAGE_PATH)
test_images()
import hashlib


filename = "e2e-tests/src/template.png"
with open(filename, "rb") as f:
    bytes = f.read() # read entire file as bytes
    readable_hash1 = hashlib.sha256(bytes).hexdigest();
    print(readable_hash1)


filename = "e2e-tests/src/testing_image.png"
with open(filename, "rb") as f:
    bytes = f.read() # read entire file as bytes
    readable_hash2 = hashlib.sha256(bytes).hexdigest();
    print(readable_hash2)


if readable_hash1 == readable_hash2:
    print("PICTURES ARE EQAUL")
import subprocess

encryption_algos = ["AES128", "AES192", "AES256"]

def run_command(command):
    try:
        result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)
        return result.stdout  # Returns the standard output of the command
    except subprocess.CalledProcessError as e:
        print("Error:", e.stderr)  # Prints the standard error in case of an error
        return None

def encrypt(algorithm, input_file, password, output_dir):
    if algorithm not in encryption_algos:
        raise ValueError(f"Invalid algorithm. Choose from {encryption_algos}")
    
    command = [
        "./RustyCryptor-base", "-E",
        "--algorithm", algorithm,
        "--input", input_file,
        "--password", password,
        "--output", output_dir
    ]
    return run_command(command)

def decrypt(algorithm, input_file, password, output_dir):
    if algorithm not in encryption_algos:
        raise ValueError(f"Invalid algorithm. Choose from {encryption_algos}")

    command = [
        "./RustyCryptor-base", "-D",
        "--algorithm", algorithm,
        "--input", input_file,
        "--password", password,
        "--output", output_dir
    ]
    return run_command(command)

def hash(input_file):
    command = [
        "./RustyCryptor-base", "-H",
        "--input", input_file
    ]
    return run_command(command)


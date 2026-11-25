import os
import sofar as sf
import torch


def main():
    filename = os.path.join("assets", "KEMAR.sofa")

    print(filename)
    with sf.SofaStream(filename) as file:
        # get information about shape of the variable Data_IR:
        # If a variable is accessed without indexing (i.e., without []) the type
        # and shape is can be obtained from the netCDF4.Variable
        data = file.Data_IR
        print(f"{data}\n")

        # read data for first receiver:
        # If a variable is accessed with indexing, only the requested data slice
        # is read directly from disk. In this case, the first channel is read.
        receiver_1 = file.Data_IR[:, 0]

    print(receiver_1)


print(f"Is CUDA supported by this system?  {torch.cuda.is_available()}")
print(f"CUDA version: {torch.version.cuda}")

cuda_id = torch.cuda.current_device()
print(f"ID of current CUDA device: {torch.cuda.current_device()}")

print(f"Name of current CUDA device: {torch.cuda.get_device_name(cuda_id)}")

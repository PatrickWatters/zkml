#!/bin/bash
#ps -ef | grep "cms_server.py" | awk '{print $2}' | xargs sudo kill
eval "$(conda shell.bash hook)"
conda activate zkml

python ../../python/converter.py --model efficientnet_lite3_fp32_2.tflite --model_output converted_model.msgpack --config_output config.msgpack --num_cols 10 --num_randoms 1024

python data_to_npy.py

python ../../python/input_converter.py --model_config converted_model.msgpack --inputs 3.npy --output example_inp.msgpack

done

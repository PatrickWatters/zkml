#!/bin/bash
#ps -ef | grep "cms_server.py" | awk '{print $2}' | xargs sudo kill
eval "$(conda shell.bash hook)"
conda activate zkml

python ../../python/converter.py --model /Users/patrickwatters/Projects/halogpu/zkml/pw_examples/tflite_models/custom_model.tflite --model_output model.msgpack --config_output config.msgpack --scale_factor 512 --k 21 --num_cols 10 --num_randoms 1024

python ../../python/input_converter.py --model_config model.msgpack --inputs 5.npy --output inp.msgpack

done

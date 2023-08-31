import tensorflow as tf
import numpy as np
import msgpack
from tensorflow import keras

mnist = tf.keras.datasets.cifar10
(images_train, labels_train), (images_test, labels_test) = mnist.load_data()

height = 280
width = 280

preprocessing_layer = tf.keras.layers.Resizing(
    height, width
)

images_test = preprocessing_layer(images_test[0])
images_test = images_test.numpy()

x = images_test
y = labels_test[0]

print(y)
x = x.flatten() / 255.
x = x.astype(np.float32)

print(x.dtype, x.shape)
np.save('3.npy', x)
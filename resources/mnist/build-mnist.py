# pip install tensorflow==1.13.2 Pillow

import tensorflow as tf
from tensorflow.keras.datasets import mnist
from PIL import Image


def main():
    (x_train_u8, y_train), (x_test_u8, y_test) = mnist.load_data()
    x_train, x_test = x_train_u8 / 255.0, x_test_u8 / 255.0

    sess = tf.keras.backend.get_session()
    model = tf.keras.models.Sequential(
        [
            tf.keras.layers.Flatten(input_shape=(28, 28)),
            tf.keras.layers.Dense(128, activation="relu"),
            tf.keras.layers.Dropout(0.2),
            tf.keras.layers.Dense(10, activation="softmax", name="output"),
        ]
    )
    model.compile(
        optimizer="adam", loss="sparse_categorical_crossentropy", metrics=["accuracy"]
    )
    model.fit(x_train, y_train, epochs=10)

    output_images = [208, 233, 666, 1115, 1234]
    output_labels = y_test[output_images]
    prob = model.predict(x_test[output_images].reshape(len(output_images), 28, 28))
    pred = prob.argmax(axis=1)
    print("Images:", output_images)
    print("Prob  :", prob)
    print("Pred  :", pred)
    print("Label :", output_labels)

    input_names = [node.op.name for node in model.inputs]
    output_names = [node.op.name for node in model.outputs]
    frozen_graph_def = tf.graph_util.convert_variables_to_constants(
        sess, sess.graph_def, output_names
    )
    with open("mnist.pb", "wb") as f:
        f.write(frozen_graph_def.SerializeToString())
    with open("mnist.pb.meta.txt", "w") as f:
        f.write(f"Input: {input_names}\n")
        f.write(f"Output: {output_names}\n")
        f.write(f"Input shape: {model.input_shape}\n")
        f.write(f"Output shape: {model.output_shape}\n")
        for idx, guess, label in zip(output_images, pred, output_labels):
            f.write(f"Test image (index, pred, label): ({idx}, {guess}, {label})\n")
    for idx in output_images:
        im = Image.fromarray(x_test_u8[idx])
        im.save(f"xtest_{idx}.png")
        with open(f"xtest_{idx}.txt", "w") as f:
            for xs in x_test_u8[idx]:
                for x in xs:
                    f.write(f"{x:4d}")
                f.write("\n")


if __name__ == "__main__":
    main()

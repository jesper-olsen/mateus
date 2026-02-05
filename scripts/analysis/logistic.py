"""
    Train a logistic regression model on a classification dataset.
    The dataset is assumed to be in a .csv.gz format file.
    Each line is a feature vector - last value is the target value.
    The dataset is assumed to have been created by gamesdb - chess positions extracted from a .pgn file.
"""
import polars as pl
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import accuracy_score, classification_report, confusion_matrix
from sklearn.preprocessing import StandardScaler
import argparse

def do_logistic_regression(fname):
    #df = pl.read_csv("Assets/ficsgamesdb_2000_standard2000_nomovetimes_394899.pgn.csv.gz", has_header=False)
    df = pl.read_csv(fname, has_header=False)
    # df.columns = ['col1', 'col2', 'col3', 'colN']  # rename the columns after loading

    X = df[:, :-1].to_numpy()  # Features (all columns except the last one)
    y = df[:, -1].to_numpy()   # Target (the last column)

    # Transform the target variable to binary classes
    # y_binary = (y > 6).astype(int)  # 1 for good (quality > 6), 0 for bad (quality <= 6)
    y_binary = y

    print("Transformed target variable (first 10 values):", y_binary[:10])

    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)

    X_train, X_test, y_train, y_test = train_test_split(X_scaled, y_binary, test_size=0.2, random_state=42)
    model = LogisticRegression(max_iter=200)
    model.fit(X_train, y_train)
    y_pred = model.predict(X_test)

    accuracy = accuracy_score(y_test, y_pred)
    print(f'Accuracy: {accuracy:.2f}')

    print('Classification Report:')
    print(classification_report(y_test, y_pred, zero_division=1))

    print('Confusion Matrix:')
    print(confusion_matrix(y_test, y_pred))

    # Inspect the weights assigned to the features
    feature_names = df.columns[:-1]  # Assuming the last column is the target
    weights = model.coef_[0]  # Get the weights for the features
    print("\nFeature Weights:")
    for feature, weight in zip(feature_names, weights):
        print(f'{feature}: {weight:.4f}')

    fname='feature_weights.npy'
    print(f"Saveing weights to {fname}")
    np.save(fname, weights)
    # loaded_weights = np.load('feature_weights.npy')
    return weights

def analyse_weights(weights):
    """Assuming the weights describe chess positions
       First 6*64 values describe 'bitboards' for white pieces, following 6*64 same for black.
    """
    def display(weights):
        for y in range(8):
            for x in range(8):
                u=int(1000*weights[y*8+x])
                print(f"{u:6d} ",end='')
            print()

    PIECES = ['P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k']

    norm=[sum(weights[0:7*64]), sum(weights[7*64:13*64])]
    print(f"Normalise: {norm}")
    for i in range(6):
        for j in range(2):
            z=i+j*6
            print(f"Piece: {PIECES[z]}")
            display(weights[z*64:(z+1)*64])

if __name__=="__main__":
    parser = argparse.ArgumentParser(description="train a logistic regression model")
    parser.add_argument('filename', type=str, help='training data (.csv.gz)')
    parser.add_argument('-t', '--train', action='store_true', help='Read the file content')
    parser.add_argument('-a', '--analyse',action='store_true', help='analyse logistic regression weights')
    args = parser.parse_args()
    filename = args.filename
    if args.train:
        weights=do_logistic_regression(filename)
    if args.analyse:
        weights = np.load(filename)
        analyse_weights(weights)


from biobase.analysis import Dna
import pandas as pd
from sklearn.linear_model import LinearRegression
from sklearn.metrics import r2_score

def main():
    df = pd.read_csv('Tm.csv')

    df["AT count"] = df["sequence"].str.count("A") + df["sequence"].str.count("T")
    df["GC count"] = df["sequence"].str.count("G") + df["sequence"].str.count("C")

    df["GC Percentage"] = df["sequence"].apply(Dna.calculate_gc_content)
    df["Inverse N"] = 1/df["sequence"].str.len()

    X_1 = df[["AT count", "GC count"]]
    y = df["Tm"]
    X_2 = df[["GC Percentage", "Inverse N"]]

    model_no_intercept = LinearRegression(fit_intercept=False)
    model_no_intercept.fit(X_1, y)

    print("No intercept model")
    # Look at model weights for model without y-intercept
    print(f"intercept = {model_no_intercept.intercept_}")
    print(f"coefficients = {model_no_intercept.coef_}\n")

    # Put coefficients together with independent variables
    for column, coefficient in zip(X_1.columns, model_no_intercept.coef_):
        print(column, coefficient)

    model_intercept = LinearRegression()
    model_intercept.fit(X_2, y)

    print("\nIntercept model")
    # Look at model weights for model with y-intercept
    print(f"intercept = {model_intercept.intercept_}")
    print(f"coefficients = {model_intercept.coef_}\n")

    # Put coefficients together with independent variables
    for column, coefficient in zip(X_2.columns, model_intercept.coef_):
        print(column, coefficient)

    r2_pred1 = model_no_intercept.score(X_1, y)
    print(f"\nModel 1 score: {r2_pred1}")

    r2_pred2 = model_intercept.score(X_2, y)
    print(f"Model 2 score: {r2_pred2}\n")

    # Original equation 1
    df["Tm original eq1"] = 2 * df["AT count"] + 4 * df["GC count"]

    # Original equation 2
    df["Tm original eq2"] = 69.30 + 0.41 * ((df["GC count"] / df["sequence"].str.len()) * 100) - 650 * (1 / df["sequence"].str.len())

    r2_eq1 = r2_score(df["Tm"], df["Tm original eq1"])
    r2_eq2 = r2_score(df["Tm"], df["Tm original eq2"])
    print(f"R2 for equation 1:\nOriginal: {r2_eq1}\nPrediction: {r2_pred1}\n")
    print(f"R2 for equation 2:\nOriginal: {r2_eq2}\nPrediction: {r2_pred2}")


if __name__ == "__main__":
    main()

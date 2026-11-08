import pandas as pd
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.metrics import mean_absolute_error, mean_squared_error
import joblib
import pickle

URL = "https://raw.githubusercontent.com/mlcollege/ai-academy/main/06-Regrese/data/ceny_domu.csv"

df = pd.read_csv(URL)

features = ["kvalita", "plocha", "rok_stavby", "rok_prodeje"]
X = df[features]
y = df["cena"]

X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.2, random_state=42
)

model = LinearRegression()

model.fit(X_train, y_train)

# print("Koeficienty (váhy jednotlivých atributů):", model.coef_)
# print("Posun (intercept - b):", model.intercept_)
# Koeficienty (váhy jednotlivých atributů): [0.61325289 0.0161942  0.01286095 0.01715594]
# Posun (intercept - b): -61.30221108146813

y_pred = model.predict(X_test)

mae = mean_absolute_error(y_test, y_pred)
mse = mean_squared_error(y_test, y_pred)

print(f"\n--- Výsledky evaluace ---")
print(f"MAE: {mae:.4f}")
print(f"MSE: {mse:.4f}")

joblib.dump(model, "model_domy.joblib")
print("\nModel byl uložen jako: model_domy.joblib")

with open("model.pkl", "wb") as f:
        pickle.dump(model, f)
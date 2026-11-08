# Domácí úkol: Lineární regrese – predikce cen domů

Tento projekt řeší úlohu predikce cen nemovitostí pomocí jednoduchého vícerozměrného lineárního modelu ($f = \mathbf{w}^T \mathbf{x} + b$) v jazyce Python.

Projekt vznikl v rámci semináře AI a algoritmů na základě datasetu z [ML College](https://raw.githubusercontent.com/mlcollege/ai-academy/main/06-Regrese/data/ceny_domu.csv).

---

## 1. Výsledné statistiky (Evaluace na testovací sadě 20 %)

Model byl vyhodnocen na 20 % neviděných testovacích dat (`test_size=0.2`, `random_state=42`):

* **MAE (Mean Absolute Error):** `0.7043` mil. Kč (průměrná absolutní odchylka je cca 704 300 Kč)
* **MSE (Mean Squared Error):** `1.1701` mil. Kč²

---

## 2. Naučený model a parametry

Model využívá 4 vstupní atributy odpovídající validační specifikaci:
1. `kvalita` – celková kvalita materiálu a provedení domu (stupnice 1–10)
2. `plocha` – obytná plocha v m²
3. `rok_stavby` – rok dokončení stavby domu
4. `rok_prodeje` – rok realizace prodeje nemovitosti

### Výsledný vzorec:
$$\text{cena} = (0{,}6133 \cdot \text{kvalita}) + (0{,}0162 \cdot \text{plocha}) + (0{,}0129 \cdot \text{rok\_stavby}) + (0{,}0172 \cdot \text{rok\_prodeje}) - 61{,}3022$$

* **Koeficienty (váhy):** `[0.61325, 0.01619, 0.01286, 0.01716]`
* **Posun (intercept $b$):** `-61.3022`

---

## 3. Struktura odevzdání

* **`main.py`** – kompletní zdrojový kód úlohy (načtení, rozdělení dat, trénování, vyhodnocení, uložení modelu).
* **`model_domy.joblib`** – natrénovaný a serializovaný model (formát Joblib).
* **`model.pkl`** – natrénovaný a serializovaný model (standardní formát Pickle).
* **`pyproject.toml`** & **`uv.lock`** – definice závislostí spravovaných nástrojem `uv`.

---

## 4. Spuštění

Pro spuštění a opětovné natrénování modelu s využitím správce balíčků `uv`:

```bash
uv run python main.py
```

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# 1. Načtení dat přímo z tvého souboru
file_path = 'Analyza-cen-du-buia.xlsx'  # případně uprav název souboru

# Načteme přesně roky 2015–2025 (11 řádků), aby se nenačetly poznámky a texty dole
df_idx = pd.read_excel(file_path, sheet_name='Indexy_2015_100', nrows=11)
df_pct = pd.read_excel(file_path, sheet_name='Mezirocni-zmeny', nrows=10)

# Výpočet celkového růstu 2015–2025 dynamicky přímo z dat
kategorie = [col for col in df_idx.columns if col not in ['Roky', 'Úhrn']]
df_growth = pd.DataFrame({
    'Kategorie': kategorie,
    'Růst 2015-2025 (%)': ((df_idx[kategorie].iloc[-1] - df_idx[kategorie].iloc[0]) / df_idx[kategorie].iloc[0]) * 100
}).reset_index(drop=True)

# 2. Úloha: Srovnání dvou kategorií (Potraviny vs. Doprava)
kat1 = 'Potraviny a nealkoholické nápoje'
kat2 = 'Doprava'

rust_kat1 = ((df_idx[kat1].iloc[-1] - df_idx[kat1].iloc[0]) / df_idx[kat1].iloc[0]) * 100
rust_kat2 = ((df_idx[kat2].iloc[-1] - df_idx[kat2].iloc[0]) / df_idx[kat2].iloc[0]) * 100

print("=== 1. Srovnání kategorií (2015–2025) ===")
print(f"  {kat1}: {rust_kat1:.1f} %")
print(f"  {kat2}: {rust_kat2:.1f} %")
print(f"  -> Více zdražila kategorie: {kat1 if rust_kat1 > rust_kat2 else kat2}\n")

# 3. Úloha: Meziroční procentní změna pro Pojištění a finanční služby
pojis_col = 'Pojištění a finanční služby'
pojis_yoy = df_pct[['Roky', pojis_col]].copy()

# Pokud jsou v tabulce hodnoty jako desetinná čísla (např. 0.018), převedeme na celá procenta
if pojis_yoy[pojis_col].abs().max() < 1.0:
    pojis_yoy[pojis_col] = pojis_yoy[pojis_col] * 100

pojis_yoy['Změna (%)'] = pojis_yoy[pojis_col].map(lambda x: f"{x:+.1f} %")

print("=== 2. Meziroční procentní změna (Pojištění a finanční služby) ===")
print(pojis_yoy[['Roky', 'Změna (%)']].to_string(index=False))
print()

# 4. Úloha: Kategorie s nejvyšším poklesem cen od 2015
nejvyssi_pokles = df_growth.sort_values(by='Růst 2015-2025 (%)').iloc[0]
print("=== 3. Kategorie s nejvyšším poklesem cen od roku 2015 ===")
print(f"  {nejvyssi_pokles['Kategorie']}: {nejvyssi_pokles['Růst 2015-2025 (%)']:.1f} %\n")

# 5. Úloha: Predikce trendu (lineární regrese na datech 2015–2024 pro 2025 a 2026)
train_df = df_idx[df_idx['Roky'] <= 2024]
slope, intercept = np.polyfit(train_df['Roky'], train_df[kat1], 1)

pred_2025 = slope * 2025 + intercept
pred_2026 = slope * 2026 + intercept
real_2025 = df_idx.loc[df_idx['Roky'] == 2025, kat1].values[0]
odchylka = pred_2025 - real_2025

print(f"=== 4. Predikce trendu pro '{kat1}' ===")
print(f"  Predikce 2025: {pred_2025:.2f}")
print(f"  Reálná hodnota 2025: {real_2025:.2f}")
print(f"  Odchylka modelu: {odchylka:+.2f} ({((odchylka) / real_2025) * 100:+.2f} %)")
print(f"  Predikce 2026: {pred_2026:.2f}\n")

# 6. Vykreslení grafu srovnání kategorií
plt.figure(figsize=(10, 5))
plt.plot(df_idx['Roky'], df_idx[kat1], marker='o', label=kat1, linewidth=2, color='#d95f02')
plt.plot(df_idx['Roky'], df_idx[kat2], marker='s', label=kat2, linewidth=2, color='#1f78b4')
plt.title('Srovnání cenového vývoje: Potraviny vs. Doprava (2015 = 100)', fontsize=13, fontweight='bold')
plt.xlabel('Rok', fontsize=11)
plt.ylabel('Index (2015 = 100)', fontsize=11)
plt.xticks(df_idx['Roky'])
plt.grid(True, linestyle='--', alpha=0.5)
plt.legend(fontsize=10)
plt.tight_layout()
plt.savefig('srovnani_kategorii.png', dpi=300)
plt.show()

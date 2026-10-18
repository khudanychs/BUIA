# Záznam komunikace s AI (Prompty)

Níže je souhrn mých zadání a promptů, kterými jsem asistenta vedl k implementaci řešení. Mým cílem bylo použít AI jako efektivního programátora, zatímco návrh architektury, volbu algoritmů a optimalizační principy jsem řídil sám na základě látky ze semináře.

---

### Prompt 1: Základní zadání, volba jazyka a srovnání BFS vs A*

> Ahoj, potřebuju napsat úkol na seminář: 2D bludiště o velikosti n x n, start je na (0,0) a cíl na (n-1, n-1). V bludišti musí být aspoň 40 % náhodných překážek velikosti 1x1. Cílem je najít nejkratší cestu nebo dokázat, že neexistuje. Základní velikost je n = 100.
>
> Chci to napsat v Rustu – zrovna se ho učím a pro tenhle typ úlohy se hodí, protože máme plnou kontrolu nad pamětí, rychlostí a neběží tam žádný garbage collector.
>
> Pro řešení chci naimplementovat dva přístupy ze stavového prostoru a porovnat je:
> 1. BFS (prohledávání do šířky) – jelikož má mřížka jednotkovou cenu kroků (nevážený graf), BFS garantuje nejkratší cestu a nemá režii prioritní fronty. Pokud se fronta vyprázdní, slouží to jako exaktní důkaz, že cesta neexistuje.
> 2. A* s Manhattan heuristikou (|x1 - x2| + |y1 - y2|) – chci vidět rozdíl v počtu prozkoumaných uzlů oproti BFS.
>
> Architektura: Zplošti mřížku do jednoho spojitého 1D vektoru a indexuj to jako `y * size + x`. Udělej k tomu hezké měření času a prozkoumaných uzlů.

---

### Prompt 2: Bonus na minimalizaci paměti (sloučení stavů a zmenšení reprezentace)

> Super, vse funguje. Teď chci vyřešit bonus na minimalizaci paměťové náročnosti.
>
> Když se dívám na kód, největší žrout paměti je ukládání předků pro rekonstrukci cesty – ukládat celou souřadnici `Option<(usize, usize)>` bere na 64bitu 16 až 24 bajtů na buňku. Přitom do buňky můžeme přijít jen ze 4 směrů!
> 
> Pojďme to zredukovat:
> - Místo celých souřadnic si budeme pamatovat jenom směr, odkud jsme přišli (nahoru, dolů, vlevo, vpravo).
> - Tím pádem můžeme sloučit `visited` a `parent` do jednoho jediného bajtu `u8` (0 = nenavštíveno, 1 = start, 2..5 = směr k předkovi). Tím ušetříme přes 90 % paměti.
> - A co samotná mřížka zdí? Šlo by ji taky zkomprimovat, abychom neplýtvali celý bajt na každé políčko, ale uložili to po jednotlivých bitech (bitset)?
>
> Napiš mi pro to paměťově úspornou verzi `CompactGrid`. Kód v češtině dobře okomentuj, ať je přesně jasné, co které bity dělají.

---

### Prompt 3: Bonus na škálování a 10minutový limit

> Teď chci vyřešit druhý bonus: škálovat velikost instance na maximum, co hardware dovolí v limitu 10 minut.
>
> U 40 % překážek jsme těsně u hranice perkolace, takže obrovské množství instancí vůbec nemá cestu a musí se prozkoumat celá komponenta. A* by tady s prioritní haldou zbytečně žral paměť a zpomaloval kvůli O(log N) operacím.
>
> Pro tohle obří škálování tedy použijeme to naše ultra-úsporné BFS.
> - Uprav generování náhodných překážek tak, ať si pro velké instance nevytváří pomocné pole všech indexů v RAM, ale generuje to rovnou do bitsetu.
> - Napiš benchmark, který postupně otestuje instance n = 100, n = 1 000, n = 5 000 a n = 10 000 (což je 100 milionů políček).
> - U každého testu spočítej a vypiš teoretickou spotřebu RAM a přesný čas generování i výpočtu cesty.

---

### Prompt 4: Formátování záznamu promptů pro odevzdání

> Super, kód funguje bez chyby a dává přesně ty výsledky, co jsme chtěli. Učitel po nás ještě chce, abychom odevzdali záznam promptů, jak jsme AI používali.
>
> Můžeš tyhle naše předchozí prompty vzít a hodit je do přehledného Markdown souboru `prompts.md` pro odevzdání? Hlavně nijak neměň jejich věcné znění ani styl, aby to byly doopravdy mé prompty, akorát hezky v MD.


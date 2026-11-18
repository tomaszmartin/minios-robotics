# MiniOS Robotics

## Programowanie bare-metal

Programowanie *bare-metal* oznacza uruchamianie programu bezpośrednio na mikrokontrolerze, bez systemu operacyjnego, takiego jak Linux.

Nasz program działa na procesorze ARM Cortex-M4 (nRF52833) znajdującym się na micro:bicie V2.1.

W Rust wykorzystujemy:

- `#![no_std]` — rezygnujemy ze standardowej biblioteki wymagającej środowiska systemowego; nadal mamy bibliotekę `core`.
- `#![no_main]` — nie używamy standardowego mechanizmu uruchamiania aplikacji.
- `#[entry]` — oznaczamy funkcję, do której kod startowy przekaże sterowanie.
- `fn main() -> !` — funkcja nigdy normalnie nie wraca do wywołującego.

### Jak Rust zarządza pamięcią?

Najważniejsze rodzaje pamięci:

| Rodzaj | Przeznaczenie |
|---|---|
| Flash | Kod programu i dane stałe |
| Stack | Lokalne dane i informacje o wywołaniach funkcji |
| Static | Dane istniejące przez cały czas działania programu |
| Heap | Pamięć przydzielana dynamicznie, jeśli skonfigurujemy alokator |

### Jak powstaje program?

Uproszczony proces:

**Kod Rust → kompilator → linker → plik wykonywalny → pamięć Flash → procesor.**

Kompilator tworzy kod maszynowy.

Linker łączy części programu i rozmieszcza je w pamięci procesora.

### Po co nam `memory.x`?

Plik `memory.x` opisuje mapę pamięci urządzenia.

Nasz nRF52833 posiada:

| Pamięć | Początek | Rozmiar |
|---|---|---|
| Flash | `0x00000000` | 512 KB |
| RAM | `0x20000000` | 128 KB |

Linker korzysta z tych informacji, żeby poprawnie rozmieścić program.

Biblioteka `cortex-m-rt` zapewnia początkową obsługę resetu, tablicę wektorów i przygotowanie pamięci przed uruchomieniem naszego `main()`.

## Sterowanie matrycą LED

### Czym jest GPIO?

GPIO (*General Purpose Input/Output*) to piny procesora, które mogą działać jako wejścia albo wyjścia.

Sterujemy nimi przez rejestry sprzętowe dostępne pod określonymi adresami pamięci.

Przykłady rejestrów portu P0 procesora nRF52833:

| Rejestr | Adres | Działanie |
|---|---|---|
| `OUT` | `0x50000504` | Stan wyjść |
| `OUTSET` | `0x50000508` | Ustawienie wybranych bitów na 1 |
| `OUTCLR` | `0x5000050C` | Ustawienie wybranych bitów na 0 |
| `DIRSET` | `0x50000518` | Konfiguracja pinów jako wyjścia |

Przykład: `1u32 << 21` tworzy maskę z ustawionym bitem numer 21.

Zapis takiej maski do `OUTSET` ustawia odpowiedni pin na HIGH bez zmiany pozostałych wyjść.

### Jak działa matryca LED?

Micro:bit posiada matrycę 5×5, czyli 25 diod.

Zamiast 25 niezależnych pinów wykorzystuje pięć wierszy i pięć kolumn.

Dioda w wybranym wierszu świeci, gdy:

- ROW = HIGH
- COL = LOW

Procesor bardzo szybko przełącza aktywny wiersz. Tę technikę nazywamy **multipleksowaniem**.

W danej chwili aktywny jest tylko jeden wiersz, ale przy odpowiedniej częstotliwości odświeżania widzimy cały obraz.

### Dlaczego widzimy cały obraz, a nie kolejne wiersze?

Metoda `show()` wielokrotnie wywołuje `drive_row()` dla wszystkich pięciu wierszy.

`delay(80_000)` przez krótki czas utrzymuje aktualnie wybrany wiersz, ponieważ stan rejestrów GPIO nie znika po zakończeniu funkcji.

Potem przełączamy następny wiersz.

Jeżeli cały cykl jest wystarczająco szybki, widzimy stabilny obraz.

**`delay(80_000)` nie oznacza 80 000 milisekund!** Jest to opóźnienie oparte na wykonywaniu instrukcji procesora, a nie dokładny timer.

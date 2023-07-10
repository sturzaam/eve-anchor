# A Constellation, Planet, and Resource harvesting problem

The only resources available in a constellation are found on planets. The decision about how many array's should be placed per resource is to made based on constellation and manufacturing requirements considerations. The crate has static data for the resource output per hour, planet, and constellation.  The requirments are exported from the game and read from the clipboard.

![Structure Material](./structure-material.png)

The list includes the quantity and up-to-date estimated valuation.  The valuation is used to determine the per-unit value when solving the system of linear inequalities for maximum total value.

```
ID	Names	Quantity	Valuation 
1	Silicate Glass	1	1011.34 
2	Smartfab Units	1	418.3 
3	Liquid Ozone	1	166.13 
4	Reactive Gas	1	195.65 
5	Noble Gas	1	363.2 
6	Industrial Fibers	1	1199.78 
7	Supertensile Plastics	1	512.55 
8	Polyaramids	1	102.93 
9	Coolant	1	607.45 
10	Condensates	1	346.7 
11	Construction Blocks	1	381.78 
12	Nanites	1	1448.58 
```


## System of Linear Inequalities

To solve the optimization problem, you simply need to have exported a list of requirements which inclues the valuation and pass in arguments for number of days `-D` and how many capsuleer outposts are anchored in the constellation `-C`.  Here is an example of what that may look like:

```bash
cargo run -- -D 7 -C FY6-NK=3 -C R2-BT6=3 -C E-ILCH=3
```

The command will maximize total value of structure materials given there are 3 outposts anchored in `FY6-NK`, `R2-BT6`, and `E-ILCH`.  The output will look like this:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/material_lp -D 7 -C FY6-NK=3 -C R2-BT6=3 -C E-ILCH=3`
Presolve 175 (-6) rows, 483 (-262) columns and 1769 (-788) elements
0  Obj -0 Primal inf 2608.7985 (13) Dual inf 4.0265257e+08 (322)
18  Obj 5.0088144e+09
Optimal - objective value 5.0088144e+09
After Postsolve, objective 5.0088144e+09, infeasibilities - dual 0 (0), primal 0 (0)
Optimal objective 5008814362 - 18 iterations time 0.002, Presolve 0.00
.----------------------------------------------------------------------------------------.
| constellation | system | planet | resource              | array_quantity | init_output |
| E-ILCH        | 02V-BK | 5      | Liquid Ozone          | 66             | 49.98       |
| E-ILCH        | 02V-BK | 8      | Noble Gas             | 66             | 33.91       |
| E-ILCH        | A5MT-B | 3      | Nanites               | 66             | 9.69        |
| E-ILCH        | A5MT-B | 7      | Noble Gas             | 66             | 30.53       |
| E-ILCH        | JD-TYH | 4      | Industrial Fibers     | 66             | 9.59        |
| E-ILCH        | JD-TYH | 5      | Construction Blocks   | 66             | 30.11       |
| E-ILCH        | MS2-V8 | 1      | Noble Gas             | 66             | 35.71       |
| E-ILCH        | MS2-V8 | 4      | Liquid Ozone          | 66             | 47.52       |
| E-ILCH        | MS2-V8 | 8      | Coolant               | 66             | 18.14       |
| E-ILCH        | SN9S-N | 4      | Liquid Ozone          | 59.23          | 42.1        |
| E-ILCH        | SN9S-N | 4      | Industrial Fibers     | 6.77           | 9.42        |
| E-ILCH        | SN9S-N | 5      | Coolant               | 66             | 18.81       |
| E-ILCH        | SN9S-N | 7      | Coolant               | 66             | 18.29       |
| FY6-NK        | 2R-KLH | 5      | Coolant               | 66             | 20.82       |
| FY6-NK        | 2R-KLH | 8      | Noble Gas             | 66             | 41.89       |
| FY6-NK        | 2R-KLH | 9      | Supertensile Plastics | 66             | 25.43       |
| FY6-NK        | 6SB-BN | 4      | Coolant               | 66             | 24.02       |
| FY6-NK        | 6SB-BN | 6      | Coolant               | 66             | 21.36       |
| FY6-NK        | 6SB-BN | 7      | Coolant               | 66             | 21.31       |
| FY6-NK        | B1D-KU | 2      | Liquid Ozone          | 66             | 53.53       |
| FY6-NK        | B1D-KU | 3      | Silicate Glass        | 66             | 12.71       |
| FY6-NK        | KSM-1T | 1      | Noble Gas             | 66             | 36.39       |
| FY6-NK        | KSM-1T | 7      | Coolant               | 66             | 22.13       |
| FY6-NK        | QFIU-K | 5      | Coolant               | 66             | 25.05       |
| FY6-NK        | YRV-MZ | 6      | Coolant               | 66             | 21.75       |
| R2-BT6        | 450I-W | 2      | Silicate Glass        | 66             | 12          |
| R2-BT6        | 450I-W | 3      | Silicate Glass        | 66             | 18.5        |
| R2-BT6        | 450I-W | 8      | Industrial Fibers     | 66             | 11.91       |
| R2-BT6        | F-UVBV | 7      | Coolant               | 66             | 20.3        |
| R2-BT6        | F-UVBV | 8      | Coolant               | 66             | 21.83       |
| R2-BT6        | OIOM-Y | 2      | Industrial Fibers     | 66             | 10.12       |
| R2-BT6        | TEIZ-C | 5      | Silicate Glass        | 66             | 11.96       |
| R2-BT6        | V-XANH | 4      | Construction Blocks   | 66             | 33.64       |
| R2-BT6        | V-XANH | 6      | Noble Gas             | 66             | 36.26       |
| R2-BT6        | VUAC-Y | 1      | Construction Blocks   | 66             | 36.59       |
| R2-BT6        | VUAC-Y | 7      | Coolant               | 66             | 21.19       |
| R2-BT6        | VUAC-Y | 8      | Coolant               | 66             | 24.81       |
'----------------------------------------------------------------------------------------'
```

##
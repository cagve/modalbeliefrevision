#!/bin/bash

# Array de strings separados por '|'
strings='''
	W: ["p"], w: p | W: ["pq"], w: pq | <2,1>
	W: ["∅"], w: ∅ | W: ["pq"], w: pq | <4,2>
	W: ["p", "q"], w: p | W: ["q", "pq"], w: pq | <2,1>
	W: ["p", "pq"], w: p | W: ["pq"], w: pq | <1,1>
	W: ["p", "∅"], w: p | W: ["pq"], w: pq | <4,1>
	W: ["p", "∅"], w: ∅ | W: ["pq"], w: pq | <4,2>
	W: ["q", "∅"], w: ∅ | W: ["q", "pq"], w: pq | <2,2>
	W: ["pq", "∅"], w: ∅ | W: ["pq"], w: pq | <2,2>
	W: ["p", "q", "pq"], w: p | W: ["q", "pq"], w: pq | <1,1>
	W: ["p", "q", "∅"], w: p | W: ["q", "pq"], w: pq | <3,1>
	W: ["p", "q", "∅"], w: ∅ | W: ["q", "pq"], w: pq | <3,2>
	W: ["p", "pq", "∅"], w: p | W: ["pq"], w: pq | <3,1>
	W: ["p", "pq", "∅"], w: ∅ | W: ["pq"], w: pq | <3,2>
	W: ["q", "pq", "∅"], w: ∅ | W: ["q", "pq"], w: pq | <1,2>
	W: ["p", "q", "pq", "∅"], w: p | W: ["q", "pq"], w: pq | <2,1>
	W: ["p", "q", "pq", "∅"], w: ∅ | W: ["q", "pq"], w: pq | <2,2>
'''

# Función para convertir cada string en formato LaTeX
convert_to_latex() {
    local string=strings
    local mu=$(echo "$string" | awk -F' | ' '{print $1}')
    local phi=$(echo "$string" | awk -F' | ' '{print $3}')
    local distance=$(echo "$string" | awk -F' | ' '{print $5}')
    
    echo "\\$mu & \\$phi & $distance \\\\"
}

# Encabezado y comienzo de la tabla LaTeX
echo '\begin{table}[h!]'
echo '\centering'
echo '\begin{tabular}{>{\centering\arraybackslash}m{6cm} >{\centering\arraybackslash}m{6cm} >{\centering\arraybackslash}m{2cm}}'
echo '\toprule'
echo '\textbf{Models of $\mu$} & \textbf{Models of $\phi$} & \textbf{Distance} \\'
echo '\midrule'

# Iterar sobre cada string y generar la fila de la tabla LaTeX
for string in "${strings[@]}"; do
    convert_to_latex "$string"
done

# Fin de la tabla LaTeX
echo '\bottomrule'
echo '\end{tabular}'
echo '\caption{Models Comparison}'
echo '\end{table}'


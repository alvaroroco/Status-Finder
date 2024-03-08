# Status Finder 🔎

Status Finder es una herramienta desarrollada en Rust diseñada para trabajar con fechas y semanas, específicamente en el formato de "status" utilizado en ciertos contextos laborales.

## ¿Qué es un "status"?

En este contexto, un "status" es un formato específico que consta de 4 dígitos:

- Los primeros dos dígitos representan el año (los últimos dos dígitos del año, por ejemplo, "23" para 2023).
- Los dos últimos dígitos representan el número de la semana en ese año, por ejemplo, "48" para la semana 48.

Por lo tanto, un "status" de "2348" se refiere a la semana 48 del año 2023.

## Características

- **Conversión de Fechas a Status**: Convierte cualquier fecha en su correspondiente "status".
- **Formatos de Fecha Flexibles**: El programa admite los siguientes formatos de fecha:
  - `%Y-%m-%d`
  - `%d/%m/%Y`
  - `%m/%d/%Y`
  - `%d-%m-%Y`
  - `%d.%m.%Y`
  
- **Obtener Status Actual**: Proporciona el "status" de la fecha actual.
- **Determinar Semana Basada en Status**: Dado un "status", encuentra el lunes y el domingo de esa semana específica.

## Uso

1. **Interfaz en Terminal (TUI)**: Si no introduces ningún argumento al ejecutar el programa, aparecerá una interfaz en la terminal que te permitirá elegir si quieres convertir de fecha a status o de status a fecha.
2. **Status Actual**: Si introduces "today", te devuelve el "status" actual.
3. **Fecha a Status**: Si introduces una fecha válida en uno de los formatos admitidos, te devuelve el "status" correspondiente a ese día.
4. **Determinar Semana por Status**: Si introduces un "status" válido, te devuelve el lunes y el domingo de esa semana específica.

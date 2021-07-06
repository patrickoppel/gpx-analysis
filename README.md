# gpx-analysis

Project to analyse GPX and TCX files

- read in all GPX and TCX files in path "./files/"
- resolve Name of File
- calculates:
  - distance (after [Vincenty's formulae](https://en.wikipedia.org/wiki/Vincenty%27s_formulae))
  - elevation gain (approximat)
  - estimated time (1h/26.5km + 10min/250m)
  - average gradient of climbs
  - direction of travel (1st half if finish is near/at start)  
- write to CSV-file

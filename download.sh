curl https://www.fao.org/fishery/static/ASFIS/ASFIS_sp.zip -o zipped.zip
unzip zipped.zip
rm ASFIS_sp_2021.xlsx zipped.zip
mv ASFIS_sp_2021.txt dataset.csv

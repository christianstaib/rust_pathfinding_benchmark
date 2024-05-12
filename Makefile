download:
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.NY.gr.gz -P data/
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-t/USA-road-t.NY.gr.gz -P data/
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.CAL.gr.gz -P data/ 
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-t/USA-road-t.CAL.gr.gz -P data/ 
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-d/USA-road-d.USA.gr.gz -P data/
	wget http://www.diag.uniroma1.it/~challenge9/data/USA-road-t/USA-road-t.USA.gr.gz -P data/
	
	gzip -fd data/*.gz



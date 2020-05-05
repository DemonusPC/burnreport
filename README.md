# Burn Report
Burn report is a web app that allows you to find out how many calories and other nutrients you have consumed. It is essentially a product database that allows you to run reports against it. 

I basically rewriten my excel spreadsheet in Rust and React.

## Local Development
```
git clone git@github.com:DemonusPC/burnreport.git
```

For the backend
```
cargo run
```

To develop on the front end start up the server and then in a separate terminal
```
cd frontend
npm install
npm run dev
```

## Release build
If you want to produce a binary just do
```
cargo build --release
```
You also need to build the frontend
```
cd frontend && npm run build
```

Then you can place the output whereever you want, assuming that the frontend is placed relatively to the binary like so:
```
/dir
  /frontend
    /build
  burnreport (binary)
```

To run it 
```
DATABASE_URL=<valid sqlite path>
```

A valid sqlite path looks like this:
```
DATABASE_URL=sqlite:///path/to/database.db
```

If this is the first time you are running the app, the database should be automatically initialized and the tables created. 

## Adding products
The product database is a simple food table and you can add products to it by navigating to `/products/add`. You can also import it as a CSV file. These are the following column schema for a valid csv file:
|name | manufacturer | kcal | kj|carbohydrates|fiber|sugar|added_sugar|starch|fat|saturated|monounsaturated|trans|protein|salt|
| ------------- |:-------------:| :-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:|:-------------:| -----:|
|example| shop| 100| 418 | 20.5 | 10.5 | 5 | 2.5 | 0 | 30 | 20.1| 1 | 0.75 | 5 | 1.25 | 

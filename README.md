### Yew web application
#### Install trunk
* `cargo install --locked trunk`
  
#### For Debug run:
* `trunk serve`
* open `http://localhost:8080`

#### After change styles run:
* `npx tailwindcss -i ./styles/input.css -o ./styles/style.css`
* `npx tailwindcss -o ./styles/style.css --minify`

#### For compile styles if needed: 
* `npx tailwindcss -i ./styles/input.css -o ./styles/style.css`

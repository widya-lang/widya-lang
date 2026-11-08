FROM nginx:alpine

# Salin konfigurasi kustom Nginx
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Salin konten situs web
COPY . /var/www/widya-lang.org

EXPOSE 80
EXPOSE 443

CMD ["nginx", "-g", "daemon off;"]

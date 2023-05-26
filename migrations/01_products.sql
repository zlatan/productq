CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE product (
  product_id uuid DEFAULT uuid_generate_v4 (),
  title VARCHAR NOT NULL,
  price FLOAT,
  currency VARCHAR,
  image_url VARCHAR NOT NULL,
  origin_url VARCHAR UNIQUE,
  last_update TIMESTAMP,
  tags JSONB,
 PRIMARY KEY (product_id)
);

INSERT INTO public.product (product_id, title, price, currency, image_url, origin_url, last_update, tags)
VALUES('a5d2f1ba-03ad-4553-a78c-ea4785ce682a'::uuid, 'Билети за концерт на Тони Стораро', 500.0, 'лв','ts.jpg', 'https://www.alo.bg/8775328', '2016-06-22 19:10:25-07', '["Билет", "Добрич"]'::jsonb);
INSERT INTO public.product(product_id, title, price, currency, image_url, origin_url, last_update, tags)
VALUES('01b926ef-cab5-4e0a-9c6e-f204f4dbd584'::uuid, 'Албум на Гошо от Почивка', 20.0, 'EU' ,'gp.jpg', 'https://www.alo.bg/8365769', '2023-04-22 19:10:25-07', '["CD", "punto com", "Malaga"]'::jsonb);
INSERT INTO public.product (product_id, title, price, currency, image_url, origin_url, last_update, tags)
VALUES('8b29c7e7-135c-4dff-86e1-6129355d3d4c'::uuid, 'Парцел за построяване на еднофамилна къща', 25000.0, 'EU','k.jpg', 'https://www.alo.bg/8543510', '2023-05-22 19:10:25-07', '["Продажба", "776 кв.м", "В регулация (УПИ)", "с. Росен"]'::jsonb);

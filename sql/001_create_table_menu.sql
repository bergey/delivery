create table restaurants (
  restaurant_id serial primary key,
  name text not null
);

create table menu_items (
  menu_item_id serial primary key,
  restaurant_id integer references restaurants(restaurant_id) not null,
  name text not null,
  price decimal(5,2) not null,
  description text,
  image text -- URL / S3
);

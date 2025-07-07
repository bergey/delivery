-- for now don't install PostGIS
alter table restaurants add column location point; -- long, lat (x, y)

-- ~ 100 km per degree near NYC, so we want at least 3 decimal places
update restaurants set location = '(-73.9713007, 40.6787101)' where restaurant_id = 1;
update restaurants set location = '(-73.9956035, 40.7368679)' where restaurant_id = 2;
update restaurants set location = '(-73.8951715, 40.7489884)' where restaurant_id = 3;

alter table restaurants alter column location set not null;
create index restaurants_location on restaurants using gist(location);

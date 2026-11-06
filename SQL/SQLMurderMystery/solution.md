# The SQL Murder Mystery Game
### My solutions to the murder mystery game
https://mystery.knightlab.com/


> A crime has taken place and the detective needs your help. The detective gave you the crime scene report, but you somehow lost it. You vaguely remember that the crime was a ​murder​ that occurred sometime on ​Jan.15, 2018​ and that it took place in ​SQL City​. Start by retrieving the corresponding crime scene report from the police department’s database.

```sql
SELECT * FROM crime_scene_report
WHERE city = 'SQL City' 
AND date = '20180115'
AND type = 'murder'
```

| date |	type |	description |	city |
| --- | --- | --- | --- |
| 20180115 |	murder |	Security footage shows that there were 2 witnesses. The first witness lives at the last house on "Northwestern Dr". The second witness, named Annabel, lives somewhere on "Franklin Ave".	| SQL City |

```sql
SELECT * FROM (
  SELECT * 
  FROM person 
  WHERE address_street_name = 'Northwestern Dr' 
  ORDER BY address_number DESC 
  LIMIT 1
)
UNION
SELECT * FROM (
  SELECT * 
  FROM person 
  WHERE name LIKE 'Annabel%' 
    AND address_street_name = 'Franklin Ave'
)
```

| id	| name |	license_id	| address_number	| address_street_name |	ssn |
| --- | --- | --- | --- | --- | --- |
| 14887	| Morty Schapiro |	118009	| 4919	| Northwestern Dr	| 111564949 |
| 16371	| Annabel Miller	| 490173	| 103	| Franklin Ave	| 318771143 |


```sql
SELECT * FROM interview
WHERE person_id IN (14887, 16371)

```

| person_id	| transcript |
| --- | --- |
| 14887 |	I heard a gunshot and then saw a man run out. He had a "Get Fit Now Gym" bag. The membership number on the bag started with "48Z". Only gold members have those bags. The man got into a car with a plate that included "H42W". |
| 16371 |	I saw the murder happen, and I recognized the killer from my gym when I was working out last week on January the 9th. |

```sql
SELECT * FROM drivers_license
WHERE id IN
   (
    SELECT license_id FROM person
    WHERE id IN
      (
        SELECT person_id FROM get_fit_now_member
        WHERE id IN
           (
            SELECT membership_id FROM get_fit_now_check_in

            WHERE check_in_date = 20180109
           ) 
         AND membership_status = 'gold'
      )
   ) 
  AND plate_number LIKE '%H42W%'
```

| id |	age |	height |	eye_color |	hair_color |	gender |	plate_number |	car_make |	car_model |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 423327	| 30	| 70	| brown	| brown	| male	| 0H42W2	| Chevrolet	| Spark LS |

```sql
SELECT * FROM person
WHERE license_id = 423327
```

| id	| name |	license_id	| address_number	| address_street_name |	ssn |
| --- | --- | --- | --- | --- | --- |
| 67318 |	Jeremy Bowers |	423327 |	530 |	Washington Pl, Apt 3A |	871539279 |

```sql
INSERT INTO solution VALUES (1, 'Jeremy Bowers');
        
        SELECT value FROM solution;
```

| value |
| --- |
| Congrats, you found the murderer! But wait, there's more... If you think you're up for a challenge, try querying the interview transcript of the murderer to find the real villain behind this crime. If you feel especially confident in your SQL skills, try to complete this final step with no more than 2 queries. Use this same INSERT statement with your new suspect to check your answer. |

```sql
SELECT * FROM interview
WHERE person_id = 67318
```

| person_id |	transcript |
| --- | --- |
| 67318 |	I was hired by a woman with a lot of money. I don't know her name but I know she's around 5'5" (65") or 5'7" (67"). She has red hair and she drives a Tesla Model S. I know that she attended the SQL Symphony Concert 3 times in December 2017. |

```sql
SELECT * FROM person
WHERE id IN (SELECT person_id FROM facebook_event_checkin
WHERE event_name = 'SQL Symphony Concert'
AND date LIKE '201712%'
GROUP BY person_id
HAVING COUNT(*) >= 3)
```

| id	| name |	license_id	| address_number	| address_street_name |	ssn |
| --- | --- | --- | --- | --- | --- |
| 24556 |	Bryan Pardo |	101191 |	703	| Machine Ln |	816663882 |
| 99716 |	Miranda Priestly |	202298 |	1883 |	Golden Ave |	987756388 |

CONFIRMATION OF MIRANDA:

```sql
SELECT * FROM income
where ssn = 987756388
```

| ssn |	annual_income |
| --- | --- |
| 987756388 |	310000 |

```sql
SELECT * FROM drivers_license
WHERE id IN (
SELECT license_id FROM person
WHERE id IN (SELECT person_id FROM facebook_event_checkin
WHERE event_name = 'SQL Symphony Concert'
AND date LIKE '201712%'
GROUP BY person_id
HAVING COUNT(*) >= 3)
)
```

| id |	age |	height |	eye_color |	hair_color |	gender |	plate_number |	car_make |	car_model |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 202298 |	68 |	66 |	green |	red |	female |	500123 |	Tesla |	Model S |

```sql
INSERT INTO solution VALUES (1, 'Miranda Priestly');
        
        SELECT value FROM solution;
```

| value |
| --- |
| Congrats, you found the brains behind the murder! Everyone in SQL City hails you as the greatest SQL detective of all time. Time to break out the champagne! |

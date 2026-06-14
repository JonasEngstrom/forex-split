# Forex Split

This crate was made to split a receipt in one currency into categories for bookkeeping in another currency, in order to facilitate bookkeeping in [YNAB](https://www.ynab.com/) while travelling in other countries.

## Use Case

Say you go out to a restaurant in Germany and pay with your Swedish card. Your receipt might be itemized as follows.

```
                          EUR
1 x Bier 0,4l            6,90
1 x Currywurst          17,50

Summe                   24,40
Trinkgeld                2,44

Kreditkarte             26,84
```

Your bank statement, on the other hand is in Swedish crowns.

```
                          SEK
Tysk restaurang       -299,28
```

You want to categorize the beer, the sausage, and the tip into alcohol, eating out, and tip categories respectively, in your Swedish crown YNAB budget. This is what this crate is for.

## Usage

### Calling the Program

Using the numbers from our [use case](#use-case) above we can call forex-split with one or two positional arguments. The first being the number from our bank statement and the second being the total from the receipt:

```bash
forex-split 299.28 26.84
```

The program will ask for any missing arguments:

```bash
forex-split 299.28
Please enter foreign total: 26.84
```

```bash
forex-split
Please enter domestic total: 299.28
Please enter foreign total: 26.84
```

The terms domestic and foregin are taken to mean domestic to your home country (SEK in our example) and foreign as in the currency of the, to you, foreign country in which you are travelling (EUR in our example).

### Running the Program

After entering the totals in both domestic and foreign currencies, you will be asked to assign sums in the foreign currency to categories until either the total equals or exceeds the total stated on the receipt or you assign the remainder of the total from the receipt to a category, called *Other*, by not explicitly typing a number in. Naming categories is optional, but if a category is named several entries can be made into the same category, giving the total value of the category at the end.

> [!NOTE]
> Note that output is rounded to two decimals.

#### Manually Assigning All Categories

```bash
forex-split 299.28 26.84
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 17.5
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): Tip 2.44
Category    Foreign subtotal    Domestic subtotal
Alcohol                 6.90                76.94
Food                   17.50               195.13
Tip                     2.44                27.21
```

#### Using the Other Category

```bash
forex-split 299.28 26.84
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 17.5
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44):
Category    Foreign subtotal    Domestic subtotal
Alcohol                 6.90                76.94
Food                   17.50               195.13
Other                   2.44                27.21
```

#### Using Unnamed Categories

```bash
forex-split 299.28 26.84
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): 6.9
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): 17.5
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): 2.44
Category    Foreign subtotal    Domestic subtotal
Unnamed Category 1      6.90                76.94
Unnamed Category 2     17.50               195.13
Unnamed Category 3      2.44                27.21
```

#### Using Unnamed Categories and the Other Category

```bash
forex-split 299.28 26.84
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): 6.9
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): 17.5
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44):
Unnamed Category 1      6.90                76.94
Unnamed Category 2     17.50               195.13
Other                   2.44                27.21
```

#### Adding Several Items to the Same Category

When you have several items on a receipt belonging to the same category is when bookkeeping while traveling can become a bit complicated. This is where forex-split can really help. Say you had two smaller dishes instead of a larger one. You can just enter them as belonging to the same category and forex-split will add them up for you.

```bash
forex-split 299.28 26.84
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 26.84): Alcohol 6.9
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 19.94): Food 9.94
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 10.00): Food 7.56
Enter subtotal in foreign currency (category subtotal) (remaining money to categorize: 2.44): Tip 2.44
Category    Foreign subtotal    Domestic subtotal
Alcohol                 6.90                76.94
Food                   17.50               195.13
Tip                     2.44                27.21
```
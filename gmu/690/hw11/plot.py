import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


def main():

    approx_energies = [
        0.054999994086498936,
        0.10999998369006136,
        0.16499996382455898,
        0.21999987907129148,
        0.274999831821349,
        0.32999986135458,
        0.384999401810457,
        0.43999986563124566,
        0.4949998674383397,
        0.549999544741144,
    ]
    step_num = [
        5540250,
        2834954,
        1555263,
        1195273,
        898719,
        618823,
        496727,
        432519,
        347541,
        509474,
    ]
    acceptance_rates = [
        0.22560119128198186,
        0.31755365342788633,
        0.38283621483954805,
        0.4343526541635258,
        0.47392566530806624,
        0.5072096544569287,
        0.5350564797162224,
        0.557573193316363,
        0.5766887935524154,
        0.5940577929393845,
    ]

    temperature = np.linspace(0.1, 1.0, len(approx_energies))

    data = {
        "Temperature": temperature,
        "Approximate Energy Values": approx_energies,
        "Number of Monte Carlo Steps performed": step_num,
        "Acceptance Rates": acceptance_rates,
    }

    df = pd.DataFrame(data)
    print(df)

    plt.figure(figsize=(8, 5))

    plt.plot(
        temperature,
        approx_energies,
        marker="o",
        linestyle="-",
        color="blue",
        label=r"Average Energy $\langle E \rangle$",
    )

    plt.title(r"Average Energy $\langle E \rangle$ vs. Temperature $T$")
    plt.xlabel(r"Temperature $T$")
    plt.ylabel(r"Average Energy $\langle E \rangle$")

    # Add a grid and legend
    plt.grid(True, linestyle="--", alpha=0.7)
    plt.legend()

    # Display the plot
    plt.show()


if __name__ == "__main__":
    main()

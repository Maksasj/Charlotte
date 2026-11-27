# Normal distribution

In probability theory and statistics, a normal distribution or Gaussian distribution is a type of continuous probability distribution for a real-valued random variable. The general form of its probability density function is

f(x)=12πσ2exp⁡(−(x−μ)22σ2).

The parameter ⁠μ⁠ is the mean or expectation of the distribution (and also its median and mode), while the parameter σ2 is the variance. The standard deviation of the distribution is the positive value ⁠σ⁠ (sigma). A random variable with a Gaussian distribution is said to be normally distributed and is called a normal deviate.

Normal distributions are important in statistics and are often used in the natural and social sciences to represent real-valued random variables whose distributions are not known. Their importance is partly due to the central limit theorem. It states that the average of many statistically independent samples (observations) of a random variable with finite mean and variance is itself a random variable—whose distribution converges to a normal distribution as the number of samples increases. Therefore, physical quantities that are expected to be the sum of many independent processes, such as measurement errors, often have distributions that are nearly normal.

Moreover, Gaussian distributions have some unique properties that are valuable in analytic studies. For instance, any linear combination of a fixed collection of independent normal deviates is a normal deviate. Many results and methods, such as propagation of uncertainty and least squares parameter fitting, can be derived analytically in explicit form when the relevant variables are normally distributed.

However, normal distributions are frequently misused in contexts where the assumption that the data are normally distributed is not met, and for which normal distribution is therefore a poor model.

A normal distribution is sometimes informally called a bell curve; however, there exist other bell-shaped distributions, such as the Cauchy, Student's t, and logistic distributions (also see § Naming, below).

The univariate probability distribution is generalized for vectors in the multivariate normal distribution and for matrices in the matrix normal distribution.

### Standard normal distribution

The simplest case of a normal distribution is known as the standard normal distribution or unit normal distribution. This is a special case when μ=0 and σ2=1, and it is described by this probability density function (or density):

φ(z)=e−z2/22π.

The variable ⁠z⁠ has a mean of 0 and a variance and standard deviation of 1. The density φ(z) has its peak value 12π at z=0 and inflection points at z=+1 and ⁠z=−1⁠.

Although the density above is most commonly known as the standard normal, a few authors have used that term to describe other versions of the normal distribution. Carl Friedrich Gauss, for example, once defined the standard normal as φ(z)=1πe−z2, which has a variance of ⁠12⁠, and Stephen Stigler once defined the standard normal as φ(z)=e−πz2, which has a simple functional form and a variance of σ2=12π.

### General normal distribution

If ⁠Z⁠ is a standard normal deviate, then X=σZ+μ will have a normal distribution with expected value ⁠μ⁠ and standard deviation ⁠σ⁠. This is equivalent to saying that the standard normal distribution ⁠Z⁠ can be scaled/stretched by a factor of ⁠σ⁠ and shifted by ⁠μ⁠ to yield a different normal distribution, called ⁠X⁠.

Conversely, if ⁠X⁠ is a normal deviate with parameters ⁠μ⁠ and σ2, then this ⁠X⁠ distribution can be re-scaled and shifted via the formula Z=(X−μ)/σ to convert it to the standard normal distribution. This variate is also called the standardized form of ⁠X⁠.

In particular, the probability density function for ⁠X⁠ can be written in terms of the standard normal distribution ⁠φ⁠ (with zero mean and unit variance):

f(x∣μ,σ2)=1σφ(x−μσ).

The probability density must be scaled by 1/σ so that the integral is still 1.

### Notation

The probability density function of the standard normal distribution is commonly denoted by the Greek letter phi, ⁠ϕ⁠. The variant form ⁠φ⁠ is also used.

The cumulative distribution function of the standard normal distribution is commonly denoted by the capital Greek letter phi, ⁠Φ⁠.

The normal distribution is often referred to as N(μ,σ2) or ⁠𝒩(μ,σ2)⁠. When a random variable ⁠X⁠ is normally distributed with mean ⁠μ⁠ and standard deviation ⁠σ⁠, one may write

X∼𝒩(μ,σ2).

### Alternative parameterizations

Some authors advocate using the precision ⁠τ⁠ as the parameter defining the width of the distribution, instead of the standard deviation ⁠σ⁠ or the variance ⁠σ2⁠. The precision is normally defined as the reciprocal of the variance, ⁠1/σ2⁠. The formula for the distribution then becomes

f(x)=τ2πe−τ(x−μ)2/2.

This choice is claimed to have advantages in numerical computations when ⁠σ⁠ is very close to zero, and simplifies formulas in some contexts, such as in the Bayesian inference of variables with multivariate normal distribution.

Alternatively, the reciprocal of the standard deviation τ′=1/σ might be defined as the precision, in which case the expression of the normal distribution becomes

f(x)=τ′2πe−(τ′)2(x−μ)2/2.

According to Stigler, this formulation is advantageous because of a much simpler and easier-to-remember formula, and simple approximate formulas for the quantiles of the distribution.

Normal distributions form an exponential family with natural parameters θ1=μσ2 and θ2=−12σ2, and natural statistics x and x2. The dual expectation parameters for normal distribution are η1 = μ and η2 = μ2 + σ2.

### Cumulative distribution function

The cumulative distribution function (CDF) of the standard normal distribution, usually denoted with the capital Greek letter ⁠Φ⁠, is the integral

Φ(x)=12π∫−∞xe−t2/2dt.

The related error function erf⁡(x) gives the probability of a random variable, with normal distribution of mean 0 and variance 1/2, falling in the range ⁠[−x,x]⁠. That is:

erf⁡(x)=1π∫−xxe−t2dt=2π∫0xe−t2dt.

These integrals cannot be expressed in terms of elementary functions, and are often said to be special functions. However, many numerical approximations are known; see below for more.

The two functions are closely related, namely

Φ(x)=12[1+erf⁡(x2)].

For a generic normal distribution with density ⁠f⁠, mean ⁠μ⁠ and variance σ2, the cumulative distribution function is

F(x)=Φ(x−μσ)=12[1+erf⁡(x−μσ2)].

The probability that x lies between a and b with a < b is thereforeP⁡(a<x≤b)=12[erf⁡(b−μσ2)−erf⁡(a−μσ2)]

The complement of the standard normal cumulative distribution function, Q(x)=1−Φ(x), is often called the Q-function, especially in engineering texts. It gives the probability that the value of a standard normal random variable ⁠X⁠ will exceed ⁠x⁠: ⁠P(X>x)⁠. Other definitions of the ⁠Q⁠-function, all of which are simple transformations of ⁠Φ⁠, are also used occasionally.

The graph of the standard normal cumulative distribution function ⁠Φ⁠ has 2-fold rotational symmetry around the point (0,1/2); that is, ⁠Φ(−x)=1−Φ(x)⁠. Its antiderivative (indefinite integral) can be expressed as follows:

∫Φ(x)dx=xΦ(x)+φ(x)+C.

An asymptotic expansion of the cumulative distribution function for large x can be derived using integration by parts:

Φ(x)=12+12πe−x2/2∑n=0∞1(2n+1)!!x2n+1.

where !! denotes the double factorial. For more, see Error function § Asymptotic expansion.

#### Taylor series representation

The Taylor series for the normal distribution ⁠φ⁠ can be derived by substituting ⁠−12x2⁠ into the Taylor series for the exponential function:

φ(x)=12π∑n=0∞(−1)nn!2nx2n

This series can be integrated term by term to obtain the Taylor series for the cumulative distribution function:

Φ(x)=12+12π∑n=0∞(−1)nn!2n(2n+1)x2n+1.

However, this series is ineffective for calculation due to slow convergence, except when ⁠x⁠ is small.

Both of these series describe entire functions, which converge for all real and complex values of ⁠x⁠.

#### Recursive computation with Taylor series

The recurrence relation for Hermite polynomials Hen(x) may be used to efficiently construct the Taylor series expansion about any point x0:

Φ(x)=∑n=0∞Φ(n)(x0)n!(x−x0)n,

where:

Φ(0)(x0)=12π∫−∞x0e−t2/2dtΦ(1)(x0)=12πe−x02/2Φ(n)(x0)=−(x0Φ(n−1)(x0)+(n−2)Φ(n−2)(x0)),n≥2.

#### Standard deviation and coverage

About 68% of values drawn from a normal distribution are within one standard deviation σ from the mean; about 95% of the values lie within two standard deviations; and about 99.7% are within three standard deviations. This is known as the 68–95–99.7 (empirical) rule, or the 3-sigma rule.

More precisely, the probability that a normal deviate lies in the range between μ−nσ and μ+nσ is given by

F(μ+nσ)−F(μ−nσ)=Φ(n)−Φ(−n)=erf⁡(n2).

To 12 significant digits, the values for n=1,2,…,6 are:

For large ⁠n⁠, one can use the approximation 1−p≈2nπen2

#### Quantile function

The quantile function of a distribution is the inverse of the cumulative distribution function. The quantile function of the standard normal distribution is called the probit function, and can be expressed in terms of the inverse error function:

Φ−1(p)=2erf−1(2p−1),p∈(0,1).

For a normal random variable with mean ⁠μ⁠ and variance σ2, the quantile function is

F−1(p)=μ+σΦ−1(p)=μ+σ2erf−1(2p−1),p∈(0,1).

The quantile Φ−1(p) of the standard normal distribution is commonly denoted as ⁠zp⁠. These values are used in hypothesis testing, construction of confidence intervals and Q–Q plots. A normal random variable ⁠X⁠ will exceed μ+zpσ with probability 1−p, and will lie outside the interval μ±zpσ with probability ⁠2(1−p)⁠. In particular, the quantile z0.975 is 1.96; therefore a normal random variable will lie outside the interval μ±1.96σ in only 5% of cases.

The following table gives the quantile zp such that ⁠X⁠ will lie in the range μ±zpσ with a specified probability ⁠p⁠. These values are useful to determine tolerance interval for sample averages and other statistical estimators with normal (or asymptotically normal) distributions. The following table shows 2erf−1(p)=Φ−1(p+12), not Φ−1(p) as defined above.

For small ⁠p⁠, the quantile function has the useful asymptotic expansion

Φ−1(p)=−ln⁡1p2−ln⁡ln⁡1p2−ln⁡(2π)+ℴ(1).

#### Using root finding to compute the quantile function

Any of the described approaches for computing the cumulative distribution function Φ(x) can be used with Newton's method (or another root-finding algorithm such as Halley's method) to find the value of ⁠x⁠ for which ⁠Φ(x)=q⁠ for some desired quantile ⁠q⁠.  For example, starting with an initial, approximately correct guess ⁠x0⁠, increasingly better approximations ⁠x1⁠, ⁠x2⁠, ... can be calculated iteratively using Newton's method with

xn=xn−1−Φ(xn−1)−qφ(xn−1).

## Properties

The normal distribution is the only distribution whose cumulants beyond the first two (i.e., other than the mean and variance) are zero. It is also the continuous distribution with the maximum entropy for a specified mean and variance. Geary has shown, assuming that the mean and variance are finite, that the normal distribution is the only distribution where the mean and variance calculated from a set of independent draws are independent of each other.

The normal distribution is a subclass of the elliptical distributions. The normal distribution is symmetric about its mean, and is non-zero over the entire real line. As such it may not be a suitable model for variables that are inherently positive or strongly skewed, such as the weight of a person or the price of a share of stock. Such variables may be better described by other distributions, such as the log-normal distribution or the Pareto distribution.

The value of the normal density is practically zero when the value ⁠x⁠ lies more than a few standard deviations away from the mean (e.g., a spread of three standard deviations covers all but 0.27% of the total distribution). Therefore, it may not be an appropriate model when one expects a significant fraction of outliers—values that lie many standard deviations away from the mean—and least squares and other statistical inference methods that are optimal for normally distributed variables often become highly unreliable when applied to such data. In those cases, a more heavy-tailed distribution should be assumed and appropriate robust statistical inference methods applied.

The Gaussian distribution belongs to the family of stable distributions which are the attractors of sums of independent, identically distributed distributions whether or not the mean or variance is finite. Except for the Gaussian which is a limiting case, all stable distributions have heavy tails and infinite variance. It is one of the few distributions that are stable and that have probability density functions that can be expressed analytically, the others being the Cauchy distribution and the Lévy distribution.

### Symmetries and derivatives

The normal distribution with density f(x) (mean ⁠μ⁠ and variance σ2>0) has the following properties:

It is symmetric around the point x=μ, which is at the same time the mode, the median and the mean of the distribution.

It is unimodal: its first derivative is positive for x<μ, negative for x>μ, and zero only at x=μ.

The area bounded by the curve and the ⁠x⁠-axis is unity (i.e. equal to one).

Its first derivative is f′(x)=−x−μσ2f(x).

Its second derivative is f″(x)=(x−μ)2−σ2σ4f(x).

Its density has two inflection points (where the second derivative of ⁠f⁠ is zero and changes sign), located one standard deviation away from the mean, namely at x=μ−σ and x=μ+σ.

Its density is log-concave.

Its density is infinitely differentiable, indeed supersmooth of order 2.

Furthermore, the density ⁠φ⁠ of the standard normal distribution (i.e. μ=0 and σ=1) also has the following properties:

Its first derivative is φ′(x)=−xφ(x).

Its second derivative is φ″(x)=(x2−1)φ(x)

More generally, its nth derivative is φ(n)(x)=(−1)nHen(x)φ(x), where Hen(x) is the nth (probabilist) Hermite polynomial.

The probability that a normally distributed variable ⁠X⁠ with known ⁠μ⁠ and σ2 is in a particular set, can be calculated given that the fraction Z=(X−μ)/σ has a standard normal distribution.

### Moments

The plain and absolute moments of a variable ⁠X⁠ are the expected values of Xp and |X|p, respectively. If the expected value ⁠μ⁠ of ⁠X⁠ is zero, these parameters are called central moments; otherwise, these parameters are called non-central moments. Usually we are interested only in moments with integer order ⁠p⁠.

If ⁠X⁠ has a normal distribution, the non-central moments exist and are finite for any ⁠p⁠ whose real part is greater than −1. For any non-negative integer ⁠p⁠, the plain central moments are:

E⁡[(X−μ)p]={0if p is odd,σp(p−1)!!if p is even.

Here n!! denotes the double factorial, that is, the product of all numbers from ⁠n⁠ to 1 that have the same parity as n.

The central absolute moments coincide with plain moments for all even orders, but are nonzero for odd orders. For any non-negative integer p,

E⁡[|X−μ|p]=σp(p−1)!!⋅{2πif p is odd1if p is even=σp⋅2p/2Γ(p+12)π.

The last formula is valid also for any non-integer p>−1. When the mean μ≠0, the plain and absolute moments can be expressed in terms of confluent hypergeometric functions 1F1 and U.

E⁡[Xp]=σp⋅(−i2)pU(−p2,12,−μ22σ2),E⁡[|X|p]=σp⋅2p/2Γ(1+p2)π1F1(−p2,12,−μ22σ2).

These expressions remain valid even when ⁠p>−1⁠ is not an integer. See also generalized Hermite polynomials.

The expectation of ⁠X⁠ conditioned on the event that ⁠X⁠ lies in an interval [a,b] is given by

E⁡[X∣a<X<b]=μ−σ2f(b)−f(a)F(b)−F(a),

where ⁠f⁠ and ⁠F⁠ respectively are the density and the cumulative distribution function of ⁠X⁠. For b=∞ this is known as the inverse Mills ratio. Note that above, density ⁠f⁠ of ⁠X⁠ is used instead of standard normal density as in inverse Mills ratio, so here we have σ2 instead of ⁠σ⁠.

### Fourier transform and characteristic function

The Fourier transform of a normal density ⁠f⁠ with mean ⁠μ⁠ and variance σ2 is

f̂(t)=∫−∞∞f(x)e−itxdx=e−iμte−12σ2t2,

where ⁠i⁠ is the imaginary unit. If the mean μ=0, the first factor is 1, and the Fourier transform is, apart from a constant factor, a normal density on the frequency domain, with mean 0 and variance ⁠1/σ2⁠. In particular, the standard normal distribution ⁠φ⁠ is an eigenfunction of the Fourier transform.

In probability theory, the Fourier transform of the probability distribution of a real-valued random variable ⁠X⁠ is closely connected to the characteristic function φX(t) of that variable, which is defined as the expected value of eitX, as a function of the real variable ⁠t⁠ (the frequency parameter of the Fourier transform). This definition can be analytically extended to a complex-value variable ⁠t⁠. The relation between both is:

φX(t)=f̂(−t).

The real and imaginary parts of f̂(t)=E⁡[e−itx]=e−iμte−12σ2t2 give:

E⁡[cos⁡(tx)]=cos⁡(μt)e−12σ2t2 and

E⁡[sin⁡(tx)]=sin⁡(μt)e−12σ2t2.

Similarly,

E⁡[cosh⁡(tx)]=cosh⁡(μt)e12σ2t2 and

E⁡[sinh⁡(tx)]=sinh⁡(μt)e12σ2t2.

These formulas evaluated at t=1 give the expected value of these basic trigonometric and hyperbolic functions over a Gaussian random variable X∼N(μ,σ2), which also could be seen as consequences of the Isserlis's theorem.

### Moment- and cumulant-generating functions

The moment generating function of a real random variable ⁠X⁠ is the expected value of etX, as a function of the real parameter ⁠t⁠. For a normal distribution with density ⁠f⁠, mean ⁠μ⁠ and variance σ2, the moment generating function exists and is equal to

M(t)=E⁡[etX]=f̂(it)=eμteσ2t2/2.

For any ⁠k⁠, the coefficient of ⁠tk/k!⁠ in the moment generating function (expressed as an exponential power series in ⁠t⁠) is the normal distribution's expected value ⁠E⁡[Xk]⁠.

The cumulant generating function is the logarithm of the moment generating function, namely

g(t)=ln⁡M(t)=μt+12σ2t2.

The coefficients of this exponential power series define the cumulants, but because this is a quadratic polynomial in  ⁠t⁠, only the first two cumulants are nonzero, namely the mean ⁠μ⁠ and the variance ⁠σ2⁠.

Some authors prefer to instead work with the characteristic function E[eitX] = eiμt − σ2t2/2 and ln E[eitX] = iμt − ⁠1/2⁠σ2t2.

### Stein operator and class

Within Stein's method the Stein operator and class of a random variable  X∼𝒩(μ,σ2) are 𝒜f(x)=σ2f′(x)−(x−μ)f(x) and ℱ the class of all absolutely continuous functions ⁠f:ℝ→ℝ⁠ such that ⁠E⁡[|f′(X)|]<∞⁠.

### Zero-variance limit

In the limit when σ2 approaches zero, the probability density f approaches zero everywhere except at μ, where it approaches ∞, while its integral remains equal to 1. An extension of the normal distribution to the case with zero variance can be defined using the Dirac delta measure δμ, although the resulting random variables are not absolutely continuous and thus do not have probability density functions.

The cumulative distribution function of such a random variable is then the Heaviside step function translated by the mean μ, namely

F(x)={0if x<μ1if x≥μ.

### Maximum entropy

Of all probability distributions over the reals with a specified finite mean ⁠μ⁠ and finite variance ⁠σ2⁠, the normal distribution N(μ,σ2) is the one with maximum entropy. To see this, let ⁠X⁠ be a continuous random variable with probability density ⁠f(x)⁠. The entropy of ⁠X⁠ is defined as

H(X)=−∫−∞∞f(x)ln⁡f(x)dx,

where f(x)log⁡f(x) is understood to be zero whenever ⁠f(x)=0⁠. This functional can be maximized, subject to the constraints that the distribution is properly normalized and has a specified mean and variance, by using variational calculus. A function with three Lagrange multipliers is defined:

L=−∫−∞∞f(x)ln⁡f(x)dx−λ0(1−∫−∞∞f(x)dx)−λ1(μ−∫−∞∞f(x)xdx)−λ2(σ2−∫−∞∞f(x)(x−μ)2dx).

At maximum entropy, a small variation δf(x) about f(x) will produce a variation δL about ⁠L⁠ which is equal to 0:

0=δL=∫−∞∞δf(x)(−ln⁡f(x)−1+λ0+λ1x+λ2(x−μ)2)dx.

Since this must hold for any small ⁠δf(x)⁠, the factor multiplying ⁠δf(x)⁠ must be zero, and solving for ⁠f(x)⁠ yields:

f(x)=exp⁡(−1+λ0+λ1x+λ2(x−μ)2).

The Lagrange constraints that ⁠f(x)⁠ is properly normalized and has the specified mean and variance are satisfied if and only if ⁠λ0⁠, ⁠λ1⁠, and ⁠λ2⁠ are chosen so that

f(x)=12πσ2e−(x−μ)22σ2.

The entropy of a normal distribution X∼N(μ,σ2) is equal to

H(X)=12(1+ln⁡2σ2π),

which is independent of the mean ⁠μ⁠.

### Central limit theorem

The central limit theorem states that under certain (fairly common) conditions, the sum of many random variables will have an approximately normal distribution. More specifically, where X1,…,Xn are independent and identically distributed random variables with the same arbitrary distribution, zero mean, and variance σ2 and ⁠Z⁠ is their

mean scaled by n

Z=n(1n∑i=1nXi)

Then, as ⁠n⁠ increases, the probability distribution of ⁠Z⁠ will tend to the normal distribution with zero mean and variance ⁠σ2⁠.

The theorem can be extended to variables (Xi) that are not independent and/or not identically distributed if certain constraints are placed on the degree of dependence and the moments of the distributions.

Many test statistics, scores, and estimators encountered in practice contain sums of certain random variables in them, and even more estimators can be represented as sums of random variables through the use of influence functions. The central limit theorem implies that those statistical parameters will have asymptotically normal distributions.

The central limit theorem also implies that certain distributions can be approximated by the normal distribution, for example:

The binomial distribution B(n,p) is approximately normal with mean np and variance np(1−p) for large ⁠n⁠ and for ⁠p⁠ not too close to 0 or 1.

The Poisson distribution with parameter ⁠λ⁠ is approximately normal with mean ⁠λ⁠ and variance ⁠λ⁠, for large values of ⁠λ⁠.

The chi-squared distribution χ2(k) is approximately normal with mean ⁠k⁠ and variance 2k, for large ⁠k⁠.

The Student's t-distribution t(ν) is approximately normal with mean 0 and variance 1 when ⁠ν⁠ is large.

Whether these approximations are sufficiently accurate depends on the purpose for which they are needed, and the rate of convergence to the normal distribution. It is typically the case that such approximations are less accurate in the tails of the distribution.

A general upper bound for the approximation error in the central limit theorem is given by the Berry–Esseen theorem, improvements of the approximation are given by the Edgeworth expansions.

This theorem can also be used to justify modeling the sum of many uniform noise sources as Gaussian noise. See AWGN.

#### Operations on a single normal variable

If ⁠X⁠ is distributed normally with mean ⁠μ⁠ and variance σ2, then

aX+b, for any real numbers ⁠a⁠ and ⁠b⁠, is also normally distributed, with mean aμ+b and variance a2σ2. That is, the family of normal distributions is closed under linear transformations.

The exponential of ⁠X⁠ is distributed log-normally: eX∼ln⁡(N(μ,σ2)).

The standard sigmoid of ⁠X⁠ is logit-normally distributed: σ(X)∼P(𝒩(μ,σ2)).

The absolute value of ⁠X⁠ has folded normal distribution: |X|∼Nf(μ,σ2). If μ=0 this is known as the half-normal distribution.

The absolute value of normalized residuals, |X−μ|/σ, has chi distribution with one degree of freedom: |X−μ|/σ∼χ1.

The square of X/σ has the noncentral chi-squared distribution with one degree of freedom: X2/σ2∼χ12(μ2/σ2). If μ=0, the distribution is called simply chi-squared.

The log-likelihood of a normal variable ⁠x⁠ is simply the log of its probability density function: ln⁡p(x)=−12(x−μσ)2−ln⁡(σ2π). Since this is a scaled and shifted square of a standard normal variable, it is distributed as a scaled and shifted chi-squared variable.

The distribution of the variable ⁠X⁠ restricted to an interval [a,b] is called the truncated normal distribution.

(X−μ)−2 has a Lévy distribution with location 0 and scale σ−2.

##### Operations on two independent normal variables

If X1 and X2 are two independent normal random variables, with means μ1, μ2 and variances σ12, σ22, then their sum X1+X2 will also be normally distributed,[proof] with mean μ1+μ2 and variance σ12+σ22.

In particular, if ⁠X⁠ and ⁠Y⁠ are independent normal deviates with zero mean and variance σ2, then X+Y and X−Y are also independent and normally distributed, with zero mean and variance 2σ2. This is a special case of the polarization identity.

If X1, X2 are two independent normal deviates with mean ⁠μ⁠ and variance σ2, and ⁠a⁠, ⁠b⁠ are arbitrary real numbers, then the variable X3=aX1+bX2−(a+b)μa2+b2+μ is also normally distributed with mean ⁠μ⁠ and variance σ2. It follows that the normal distribution is stable (with exponent α=2).

If Xk∼𝒩(mk,σk2), k∈{0,1} are normal distributions, then their normalized geometric mean 1∫ℝnX0α(x)X11−α(x)dxX0αX11−α is a normal distribution 𝒩(mα,σα2) with mα=αm0σ12+(1−α)m1σ02ασ12+(1−α)σ02 and σα2=σ02σ12ασ12+(1−α)σ02.

##### Operations on two independent standard normal variables

If X1 and X2 are two independent standard normal random variables with mean 0 and variance 1, then

Their sum and difference is distributed normally with mean zero and variance two: X1±X2∼𝒩(0,2).

Their product Z=X1X2 follows the product distribution with density function fZ(z)=π−1K0(|z|) where K0 is the modified Bessel function of the second kind. This distribution is symmetric around zero, unbounded at z=0, and has the characteristic function ϕZ(t)=(1+t2)−1/2.

Their ratio follows the standard Cauchy distribution: X1/X2∼Cauchy⁡(0,1).

Their Euclidean norm X12+X22 has the Rayleigh distribution.

#### Operations on multiple independent normal variables

Any linear combination of independent normal deviates is a normal deviate.

If X1,X2,…,Xn are independent standard normal random variables, then the sum of their squares has the chi-squared distribution with ⁠n⁠ degrees of freedom X12+⋯+Xn2∼χn2.

If X1,X2,…,Xn are independent normally distributed random variables with means ⁠μ⁠ and variances σ2, then their sample mean is independent from the sample standard deviation, which can be demonstrated using Basu's theorem or Cochran's theorem. The ratio of these two quantities will have the Student's t-distribution with n−1 degrees of freedom: t=X‾−μS/n=1n(X1+⋯+Xn)−μ1n(n−1)[(X1−X‾)2+⋯+(Xn−X‾)2]∼tn−1.

If X1,X2,…,Xn, Y1,Y2,…,Ym are independent standard normal random variables, then the ratio of their normalized sums of squares will have the F-distribution with (n, m) degrees of freedom: F=(X12+X22+⋯+Xn2)/n(Y12+Y22+⋯+Ym2)/m∼Fn,m.

#### Operations on multiple correlated normal variables

A quadratic form of a normal vector, i.e. a quadratic function q=∑xi2+∑xj+c of multiple independent or correlated normal variables, is a generalized chi-square variable.

### Operations on the density function

The split normal distribution is most directly defined in terms of joining scaled sections of the density functions of different normal distributions and rescaling the density to integrate to one. The truncated normal distribution results from rescaling a section of a single density function.

### Infinite divisibility and Cramér's theorem

For any positive integer n, any normal distribution with mean ⁠μ⁠ and variance σ2 is the distribution of the sum of n independent normal deviates, each with mean μn and variance σ2n. This property is called infinite divisibility.

Conversely, if X1 and X2 are independent random variables and their sum X1+X2 has a normal distribution, then both X1 and X2 must be normal deviates.

This result is known as Cramér's decomposition theorem, and is equivalent to saying that the convolution of two distributions is normal if and only if both are normal. Cramér's theorem implies that a linear combination of independent non-Gaussian variables will never have an exactly normal distribution, although it may approach it arbitrarily closely.

### The Kac–Bernstein theorem

The Kac–Bernstein theorem states that if X and ⁠Y⁠ are independent and X+Y and X−Y are also independent, then both X and Y must necessarily have normal distributions.

More generally, if X1,…,Xn are independent random variables, then two distinct linear combinations ∑akXk and ∑bkXkwill be independent if and only if all Xk are normal and ∑akbkσk2=0, where σk2 denotes the variance of Xk.

### Extensions

The notion of normal distribution, being one of the most important distributions in probability theory, has been extended far beyond the standard framework of the univariate (that is one-dimensional) case (Case 1). All these extensions are also called normal or Gaussian laws, so a certain ambiguity in names exists.

The multivariate normal distribution describes the Gaussian law in the k-dimensional Euclidean space. A vector X ∈ Rk is multivariate-normally distributed if any linear combination of its components Σkj=1aj Xj has a (univariate) normal distribution. The variance of X is a k × k symmetric positive-definite matrix V. The multivariate normal distribution is a special case of the elliptical distributions. As such, its iso-density loci in the k = 2 case are ellipses and in the case of arbitrary k are ellipsoids.

Rectified Gaussian distribution a rectified version of normal distribution with all the negative elements reset to 0.

Complex normal distribution deals with the complex normal vectors. A complex vector X ∈ Ck is said to be normal if both its real and imaginary components jointly possess a 2k-dimensional multivariate normal distribution. The variance-covariance structure of X is described by two matrices: the variance matrix Γ, and the relation matrix C.

Matrix normal distribution describes the case of normally distributed matrices.

Gaussian processes are the normally distributed stochastic processes. These can be viewed as elements of some infinite-dimensional Hilbert space H, and thus are the analogues of multivariate normal vectors for the case k = ∞. A random element h ∈ H is said to be normal if for any constant a ∈ H the scalar product (a, h) has a (univariate) normal distribution. The variance structure of such Gaussian random element can be described in terms of the linear covariance operator K: H → H. Several Gaussian processes became popular enough to have their own names:

Brownian motion;

Brownian bridge; and

Ornstein–Uhlenbeck process.

Gaussian q-distribution is an abstract mathematical construction that represents a q-analogue of the normal distribution.

the q-Gaussian is an analogue of the Gaussian distribution, in the sense that it maximises the Tsallis entropy, and is one type of Tsallis distribution. This distribution is different from the Gaussian q-distribution above.

The Kaniadakis κ-Gaussian distribution is a generalization of the Gaussian distribution which arises from the Kaniadakis statistics, being one of the Kaniadakis distributions.

A random variable X has a two-piece normal distribution if it has a distribution

fX(x)={N(μ,σ12), if x≤μN(μ,σ22), if x≥μ

where μ is the mean and σ21  and σ22  are the variances of the distribution to the left and right of the mean respectively.

The mean E(X), variance V(X), and third central moment T(X) of this distribution have been determined

E⁡(X)=μ+2π(σ2−σ1),V⁡(X)=(1−2π)(σ2−σ1)2+σ1σ2,T⁡(X)=2π(σ2−σ1)[(4π−1)(σ2−σ1)2+σ1σ2].

One of the main practical uses of the Gaussian law is to model the empirical distributions of many different random variables encountered in practice. In such case a possible extension would be a richer family of distributions, having more than two parameters and therefore being able to fit the empirical distribution more accurately. The examples of such extensions are:

Pearson distribution — a four-parameter family of probability distributions that extend the normal law to include different skewness and kurtosis values.

The generalized normal distribution, also known as the exponential power distribution, allows for distribution tails with thicker or thinner asymptotic behaviors.

### Estimation of parameters

It is often the case that we do not know the parameters of the normal distribution, but instead want to estimate them. That is, having a sample (x1,…,xn) from a normal 𝒩(μ,σ2) population we would like to learn the approximate values of parameters ⁠μ⁠ and σ2. The standard approach to this problem is the maximum likelihood method, which requires maximization of the log-likelihood function:

ln⁡ℒ(μ,σ2)=∑i=1nln⁡f(xi∣μ,σ2)=−n2ln⁡(2π)−n2ln⁡σ2−12σ2∑i=1n(xi−μ)2.

Taking derivatives with respect to ⁠μ⁠ and σ2 and solving the resulting system of first order conditions yields the maximum likelihood estimates:

μ̂=x‾≡1n∑i=1nxi,σ̂2=1n∑i=1n(xi−x‾)2.

Then ln⁡ℒ(μ̂,σ̂2) is as follows:

ln⁡ℒ(μ̂,σ̂2)=−n2[ln⁡(2πσ̂2)+1]

#### Sample mean

Estimator μ̂ is called the sample mean, since it is the arithmetic mean of all observations. The statistic x‾ is complete and sufficient for ⁠μ⁠, and therefore by the Lehmann–Scheffé theorem, μ̂ is the uniformly minimum variance unbiased (UMVU) estimator. In finite samples it is distributed normally:

μ̂∼𝒩(μ,σ2/n).

The variance of this estimator is equal to the μμ-element of the inverse Fisher information matrix ℐ−1. This implies that the estimator is finite-sample efficient. Of practical importance is the standard error of μ̂ being proportional to 1/n, that is, if one wishes to decrease the standard error by a factor of 10, one must increase the number of points in the sample by a factor of 100. This fact is widely used in determining sample sizes for opinion polls and the number of trials in Monte Carlo simulations.

From the standpoint of the asymptotic theory, μ̂ is consistent, that is, it converges in probability to ⁠μ⁠ as n→∞. The estimator is also asymptotically normal, which is a simple corollary of it being normal in finite samples:

n(μ̂−μ)→d𝒩(0,σ2).

#### Sample variance

The estimator σ̂2 is called the sample variance, since it is the variance of the sample ((x1,…,xn)). In practice, another estimator is often used instead of the σ̂2. This other estimator is denoted  s2, and is also called the sample variance, which represents a certain ambiguity in terminology; its square root ⁠s⁠ is called the sample standard deviation. The estimator s2 differs from σ̂2 by having (n − 1) instead of n in the denominator (the so-called Bessel's correction):

s2=nn−1σ̂2=1n−1∑i=1n(xi−x‾)2.

The difference between s2 and σ̂2 becomes negligibly small for large n's. In finite samples however, the motivation behind the use of s2 is that it is an unbiased estimator of the underlying parameter σ2, whereas σ̂2 is biased. Also, by the Lehmann–Scheffé theorem the estimator s2 is uniformly minimum variance unbiased (UMVU), which makes it the "best" estimator among all unbiased ones. However it can be shown that the biased estimator σ̂2 is better than the s2 in terms of the mean squared error (MSE) criterion. In finite samples both s2 and σ̂2 have scaled chi-squared distribution with (n − 1) degrees of freedom:

s2∼σ2n−1⋅χn−12,σ̂2∼σ2n⋅χn−12.

The first of these expressions shows that the variance of s2 is equal to 2σ4/(n−1), which is slightly greater than the σσ-element of the inverse Fisher information matrix ℐ−1, which is 2σ4/n. Thus, s2 is not an efficient estimator for σ2, and moreover, since s2 is UMVU, we can conclude that the finite-sample efficient estimator for σ2 does not exist.

Applying the asymptotic theory, both estimators s2 and σ̂2 are consistent, that is they converge in probability to σ2 as the sample size n→∞. The two estimators are also both asymptotically normal:

n(σ̂2−σ2)≃n(s2−σ2)→d𝒩(0,2σ4).

In particular, both estimators are asymptotically efficient for  σ2.

### Confidence intervals

By Cochran's theorem, for normal distributions the sample mean μ̂ and the sample variance s2 are independent, which means there can be no gain in considering their joint distribution. There is also a converse theorem: if in a sample the sample mean and sample variance are independent, then the sample must have come from the normal distribution. The independence between μ̂ and s can be employed to construct the so-called t-statistic:

t=μ̂−μs/n=x‾−μ1n(n−1)∑(xi−x‾)2∼tn−1

This quantity t has the Student's t-distribution with (n − 1) degrees of freedom, and it is an ancillary statistic (independent of the value of the parameters). Inverting the distribution of this t-statistics will allow us to construct the confidence interval for μ; similarly, inverting the χ2 distribution of the statistic s2 will give us the confidence interval for σ2:

μ∈[μ̂−tn−1,1−α/2sn,μ̂+tn−1,1−α/2sn]

σ2∈[n−1χn−1,1−α/22s2,n−1χn−1,α/22s2]

where tk,p and χ 2k,p  are the pth quantiles of the t- and χ2-distributions respectively. These confidence intervals are of the confidence level 1 − α, meaning that the true values μ and σ2 fall outside of these intervals with probability (or significance level) α. In practice people usually take α = 5%, resulting in the 95% confidence intervals. The confidence interval for σ can be found by taking the square root of the interval bounds for σ2.

Approximate formulas can be derived from the asymptotic distributions of μ̂ and s2:

μ∈[μ̂−|zα/2|ns,μ̂+|zα/2|ns]

σ2∈[s2−2|zα/2|ns2,s2+2|zα/2|ns2]

The approximate formulas become valid for large values of n, and are more convenient for the manual calculation since the standard normal quantiles zα/2 do not depend on n. In particular, the most popular value of α = 5%, results in |z0.025| = 1.96.

### Normality tests

Normality tests assess the likelihood that the given data set {x1, ..., xn} comes from a normal distribution. Typically the null hypothesis H0 is that the observations are distributed normally with unspecified mean μ and variance σ2, versus the alternative Ha that the distribution is arbitrary. Many tests (over 40) have been devised for this problem. The more prominent of them are outlined below:

Diagnostic plots are more intuitively appealing but subjective at the same time, as they rely on informal human judgement to accept or reject the null hypothesis.

Q–Q plot, also known as normal probability plot or rankit plot—is a plot of the sorted values from the data set against the expected values of the corresponding quantiles from the standard normal distribution. That is, it is a plot of point of the form (Φ−1(pk), x(k)), where plotting points pk are equal to pk = (k − α)/(n + 1 − 2α) and α is an adjustment constant, which can be anything between 0 and 1. If the null hypothesis is true, the plotted points should approximately lie on a straight line.

P–P plot – similar to the Q–Q plot, but used much less frequently. This method consists of plotting the points (Φ(z(k)), pk), where z(k)=(x(k)−μ̂)/σ̂. For normally distributed data this plot should lie on a straight line between (0, 0) and (1, 1).

Goodness-of-fit tests:

Moment-based tests:

D'Agostino's K-squared test

Jarque–Bera test

Shapiro–Wilk test: This is based on the line in the Q–Q plot having the slope of σ. The test compares the least squares estimate of that slope with the value of the sample variance, and rejects the null hypothesis if these two quantities differ significantly.

Tests based on the empirical distribution function:

Anderson–Darling test

Lilliefors test (an adaptation of the Kolmogorov–Smirnov test)

### Bayesian analysis of the normal distribution

Bayesian analysis of normally distributed data is complicated by the many different possibilities that may be considered:

Either the mean, or the variance, or neither, may be considered a fixed quantity.

When the variance is unknown, analysis may be done directly in terms of the variance, or in terms of the precision, the reciprocal of the variance. The reason for expressing the formulas in terms of precision is that the analysis of most cases is simplified.

Both univariate and multivariate cases need to be considered.

Either conjugate or improper prior distributions may be placed on the unknown variables.

An additional set of cases occurs in Bayesian linear regression, where in the basic model the data is assumed to be normally distributed, and normal priors are placed on the regression coefficients. The resulting analysis is similar to the basic cases of independent identically distributed data.

The formulas for the non-linear-regression cases are summarized in the conjugate prior article.

##### Scalar form

The following auxiliary formula is useful for simplifying the posterior update equations, which otherwise become fairly tedious.

a(x−y)2+b(x−z)2=(a+b)(x−ay+bza+b)2+aba+b(y−z)2

This equation rewrites the sum of two quadratics in x by expanding the squares, grouping the terms in x, and completing the square. Note the following about the complex constant factors attached to some of the terms:

The factor ay+bza+b has the form of a weighted average of y and z.

aba+b=11a+1b=(a−1+b−1)−1. This shows that this factor can be thought of as resulting from a situation where the reciprocals of quantities a and b add directly, so to combine a and b themselves, it is necessary to reciprocate, add, and reciprocate the result again to get back into the original units. This is exactly the sort of operation performed by the harmonic mean, so it is not surprising that aba+b is one-half the harmonic mean of a and b.

##### Vector form

A similar formula can be written for the sum of two vector quadratics: If x, y, z are vectors of length k, and A and B are symmetric, invertible matrices of size k×k, then

(𝐲−𝐱)′𝐀(𝐲−𝐱)+(𝐱−𝐳)′𝐁(𝐱−𝐳)=(𝐱−𝐜)′(𝐀+𝐁)(𝐱−𝐜)+(𝐲−𝐳)′(𝐀−1+𝐁−1)−1(𝐲−𝐳)

where

𝐜=(𝐀+𝐁)−1(𝐀𝐲+𝐁𝐳)

The form x′ A x is called a quadratic form and is a scalar:

𝐱′𝐀𝐱=∑i,jaijxixj

In other words, it sums up all possible combinations of products of pairs of elements from x, with a separate coefficient for each. In addition, since xixj=xjxi, only the sum aij+aji matters for any off-diagonal elements of A, and there is no loss of generality in assuming that A is symmetric. Furthermore, if A is symmetric, then the form 𝐱′𝐀𝐲=𝐲′𝐀𝐱.

#### Sum of differences from the mean

Another useful formula is as follows:

∑i=1n(xi−μ)2=∑i=1n(xi−x¯)2+n(x¯−μ)2

where x¯=1n∑i=1nxi.

### With known variance

For a set of i.i.d. normally distributed data points X of size n where each individual point x follows x∼𝒩(μ,σ2) with known variance σ2, the conjugate prior distribution is also normally distributed.

This can be shown more easily by rewriting the variance as the precision, i.e. using τ = 1/σ2. Then if x∼𝒩(μ,1/τ) and μ∼𝒩(μ0,1/τ0), we proceed as follows.

First, the likelihood function is (using the formula above for the sum of differences from the mean):

p(𝐗∣μ,τ)=∏i=1nτ2πexp⁡(−12τ(xi−μ)2)=(τ2π)n/2exp⁡(−12τ∑i=1n(xi−μ)2)=(τ2π)n/2exp⁡[−12τ(∑i=1n(xi−x¯)2+n(x¯−μ)2)].

Then, we proceed as follows:

p(μ∣𝐗)∝p(𝐗∣μ)p(μ)=(τ2π)n/2exp⁡[−12τ(∑i=1n(xi−x¯)2+n(x¯−μ)2)]τ02πexp⁡(−12τ0(μ−μ0)2)∝exp⁡(−12(τ(∑i=1n(xi−x¯)2+n(x¯−μ)2)+τ0(μ−μ0)2))∝exp⁡(−12(nτ(x¯−μ)2+τ0(μ−μ0)2))=exp⁡(−12(nτ+τ0)(μ−nτx¯+τ0μ0nτ+τ0)2+nττ0nτ+τ0(x¯−μ0)2)∝exp⁡(−12(nτ+τ0)(μ−nτx¯+τ0μ0nτ+τ0)2)

In the above derivation, we used the formula above for the sum of two quadratics and eliminated all constant factors not involving μ. The result is the kernel of a normal distribution, with mean nτx¯+τ0μ0nτ+τ0 and precision nτ+τ0, i.e.

p(μ∣𝐗)∼𝒩(nτx¯+τ0μ0nτ+τ0,1nτ+τ0)

This can be written as a set of Bayesian update equations for the posterior parameters in terms of the prior parameters:

τ0′=τ0+nτμ0′=nτx¯+τ0μ0nτ+τ0x¯=1n∑i=1nxi

That is, to combine n data points with total precision of nτ (or equivalently, total variance of n/σ2) and mean of values x¯, derive a new total precision simply by adding the total precision of the data to the prior total precision, and form a new mean through a precision-weighted average, i.e. a weighted average of the data mean and the prior mean, each weighted by the associated total precision. This makes logical sense if the precision is thought of as indicating the certainty of the observations: In the distribution of the posterior mean, each of the input components is weighted by its certainty, and the certainty of this distribution is the sum of the individual certainties. (For the intuition of this, compare the expression "the whole is (or is not) greater than the sum of its parts". In addition, consider that the knowledge of the posterior comes from a combination of the knowledge of the prior and likelihood, so it makes sense that we are more certain of it than of either of its components.)

The above formula reveals why it is more convenient to do Bayesian analysis of conjugate priors for the normal distribution in terms of the precision. The posterior precision is simply the sum of the prior and likelihood precisions, and the posterior mean is computed through a precision-weighted average, as described above. The same formulas can be written in terms of variance by reciprocating all the precisions, yielding the more ugly formulas

σ02′=1nσ2+1σ02μ0′=nx¯σ2+μ0σ02nσ2+1σ02x¯=1n∑i=1nxi

#### With known mean

For a set of i.i.d. normally distributed data points X of size n where each individual point x follows x∼𝒩(μ,σ2) with known mean μ, the conjugate prior of the variance has an inverse gamma distribution or a scaled inverse chi-squared distribution. The two are equivalent except for having different parameterizations. Although the inverse gamma is more commonly used, we use the scaled inverse chi-squared for the sake of convenience. The prior for σ2 is as follows:

p(σ2∣ν0,σ02)=(σ02ν02)ν0/2Γ(ν02)exp⁡[−ν0σ022σ2](σ2)1+ν02∝exp⁡[−ν0σ022σ2](σ2)1+ν02

The likelihood function from above, written in terms of the variance, is:

p(𝐗∣μ,σ2)=(12πσ2)n/2exp⁡[−12σ2∑i=1n(xi−μ)2]=(12πσ2)n/2exp⁡[−S2σ2]

where

S=∑i=1n(xi−μ)2.

Then:

p(σ2∣𝐗)∝p(𝐗∣σ2)p(σ2)=(12πσ2)n/2exp⁡[−S2σ2](σ02ν02)ν02Γ(ν02)exp⁡[−ν0σ022σ2](σ2)1+ν02∝(1σ2)n/21(σ2)1+ν02exp⁡[−S2σ2+−ν0σ022σ2]=1(σ2)1+ν0+n2exp⁡[−ν0σ02+S2σ2]

The above is also a scaled inverse chi-squared distribution where

ν0′=ν0+nν0′σ02′=ν0σ02+∑i=1n(xi−μ)2

or equivalently

ν0′=ν0+nσ02′=ν0σ02+∑i=1n(xi−μ)2ν0+n

Reparameterizing in terms of an inverse gamma distribution, the result is:

α′=α+n2β′=β+∑i=1n(xi−μ)22

#### With unknown mean and unknown variance

For a set of i.i.d. normally distributed data points X of size n where each individual point x follows x∼𝒩(μ,σ2) with unknown mean μ and unknown variance σ2, a combined (multivariate) conjugate prior is placed over the mean and variance, consisting of a normal-inverse-gamma distribution.

Logically, this originates as follows:

From the analysis of the case with unknown mean but known variance, we see that the update equations involve sufficient statistics computed from the data consisting of the mean of the data points and the total variance of the data points, computed in turn from the known variance divided by the number of data points.

From the analysis of the case with unknown variance but known mean, we see that the update equations involve sufficient statistics over the data consisting of the number of data points and sum of squared deviations.

Keep in mind that the posterior update values serve as the prior distribution when further data is handled. Thus, we should logically think of our priors in terms of the sufficient statistics just described, with the same semantics kept in mind as much as possible.

To handle the case where both mean and variance are unknown, we could place independent priors over the mean and variance, with fixed estimates of the average mean, total variance, number of data points used to compute the variance prior, and sum of squared deviations. Note however that in reality, the total variance of the mean depends on the unknown variance, and the sum of squared deviations that goes into the variance prior (appears to) depend on the unknown mean. In practice, the latter dependence is relatively unimportant: Shifting the actual mean shifts the generated points by an equal amount, and on average the squared deviations will remain the same. This is not the case, however, with the total variance of the mean: As the unknown variance increases, the total variance of the mean will increase proportionately, and we would like to capture this dependence.

This suggests that we create a conditional prior of the mean on the unknown variance, with a hyperparameter specifying the mean of the pseudo-observations associated with the prior, and another parameter specifying the number of pseudo-observations. This number serves as a scaling parameter on the variance, making it possible to control the overall variance of the mean relative to the actual variance parameter. The prior for the variance also has two hyperparameters, one specifying the sum of squared deviations of the pseudo-observations associated with the prior, and another specifying once again the number of pseudo-observations. Each of the priors has a hyperparameter specifying the number of pseudo-observations, and in each case this controls the relative variance of that prior. These are given as two separate hyperparameters so that the variance (aka the confidence) of the two priors can be controlled separately.

This leads immediately to the normal-inverse-gamma distribution, which is the product of the two distributions just defined, with conjugate priors used (an inverse gamma distribution over the variance, and a normal distribution over the mean, conditional on the variance) and with the same four parameters just defined.

The priors are normally defined as follows:

p(μ∣σ2;μ0,n0)∼𝒩(μ0,σ2/n0)p(σ2;ν0,σ02)∼Iχ2(ν0,σ02)=IG(ν0/2,ν0σ02/2)

The update equations can be derived, and look as follows:

x¯=1n∑i=1nxiμ0′=n0μ0+nx¯n0+nn0′=n0+nν0′=ν0+nν0′σ02′=ν0σ02+∑i=1n(xi−x¯)2+n0nn0+n(μ0−x¯)2The respective numbers of pseudo-observations add the number of actual observations to them. The new mean hyperparameter is once again a weighted average, this time weighted by the relative numbers of observations. Finally, the update for ν0′σ02′ is similar to the case with known mean, but in this case the sum of squared deviations is taken with respect to the observed data mean rather than the true mean, and as a result a new interaction term needs to be added to take care of the additional error source stemming from the deviation between prior and data mean.

## Occurrence and applications

The occurrence of normal distribution in practical problems can be loosely classified into four categories:

Exactly normal distributions;

Approximately normal laws, for example when such approximation is justified by the central limit theorem; and

Distributions modeled as normal – the normal distribution being the distribution with maximum entropy for a given mean and variance.

Regression problems – the normal distribution being found after systematic effects have been modeled sufficiently well.

### Exact normality

A normal distribution occurs in some physical theories:

The velocity distribution of independently moving and perfectly elastic spheres, which is a consequence of Maxwell's Dynamical Theory of Gases, Part I (1860).

The ground state wave function in position space of the quantum harmonic oscillator.

The position of a particle that experiences diffusion. If initially the particle is located at a specific point (that is its probability distribution is the Dirac delta function), then after time t its location is described by a normal distribution with variance t, which satisfies the diffusion equation ∂∂tf(x,t)=12∂2∂x2f(x,t). If the initial location is given by a certain density function g(x), then the density at time t is the convolution of g and the normal probability density function.

### Approximate normality

Approximately normal distributions occur in many situations, as explained by the central limit theorem. When the outcome is produced by many small effects acting additively and independently, its distribution will be close to normal. The normal approximation will not be valid if the effects act multiplicatively (instead of additively), or if there is a single external influence that has a considerably larger magnitude than the rest of the effects.

In counting problems, where the central limit theorem includes a discrete-to-continuum approximation and where infinitely divisible and decomposable distributions are involved, such as

Binomial random variables, associated with binary response variables;

Poisson random variables, associated with rare events;

Thermal radiation has a Bose–Einstein distribution on very short time scales, and a normal distribution on longer timescales due to the central limit theorem.

### Assumed normality

I can only recognize the occurrence of the normal curve – the Laplacian curve of errors – as a very abnormal phenomenon. It is roughly approximated to in certain distributions; for this reason, and on account for its beautiful simplicity, we may, perhaps, use it as a first approximation, particularly in theoretical investigations.

There are statistical methods to empirically test that assumption; see the above Normality tests section.

In biology, the logarithm of various variables tend to have a normal distribution, that is, they tend to have a log-normal distribution (after separation on male/female subpopulations), with examples including:

Measures of size of living tissue (length, height, skin area, weight);

The length of inert appendages (hair, claws, nails, teeth) of biological specimens, in the direction of growth; presumably the thickness of tree bark also falls under this category;

Certain physiological measurements, such as blood pressure of adult humans.

In finance, in particular the Black–Scholes model, changes in the logarithm of exchange rates, price indices, and stock market indices are assumed normal (these variables behave like compound interest, not like simple interest, and so are multiplicative). Some mathematicians such as Benoit Mandelbrot have argued that log-Levy distributions, which possess heavy tails, would be a more appropriate model, in particular for the analysis for stock market crashes. The use of the assumption of normal distribution occurring in financial models has also been criticized by Nassim Nicholas Taleb in his works.

Measurement errors in physical experiments are often modeled by a normal distribution. This use of a normal distribution does not imply that one is assuming the measurement errors are normally distributed, rather using the normal distribution produces the most conservative predictions possible given only knowledge about the mean and variance of the errors.

In standardized testing, results can be made to have a normal distribution by either selecting the number and difficulty of questions (as in the IQ test) or transforming the raw test scores into output scores by fitting them to the normal distribution. For example, the SAT's traditional range of 200–800 is based on a normal distribution with a mean of 500 and a standard deviation of 100.

Many scores are derived from the normal distribution, including percentile ranks (percentiles or quantiles), normal curve equivalents, stanines, z-scores, and T-scores. Additionally, some behavioral statistical procedures assume that scores are normally distributed; for example, t-tests and ANOVAs. Bell curve grading assigns relative grades based on a normal distribution of scores.

In hydrology the distribution of long duration river discharge or rainfall, e.g. monthly and yearly totals, is often thought to be practically normal according to the central limit theorem. The plot on the right illustrates an example of fitting the normal distribution to ranked October rainfalls showing the 90% confidence belt based on the binomial distribution. The rainfall data are represented by plotting positions as part of the cumulative frequency analysis.

### Methodological problems and peer review

John Ioannidis argued that using normally distributed standard deviations as standards for validating research findings leave falsifiable predictions about phenomena that are not normally distributed untested. This includes, for example, phenomena that only appear when all necessary conditions are present and one cannot be a substitute for another in an addition-like way and phenomena that are not randomly distributed. Ioannidis argues that standard deviation-centered validation gives a false appearance of validity to hypotheses and theories where some but not all falsifiable predictions are normally distributed since the portion of falsifiable predictions that there is evidence against may and in some cases are in the non-normally distributed parts of the range of falsifiable predictions, as well as baselessly dismissing hypotheses for which none of the falsifiable predictions are normally distributed as if they were unfalsifiable when in fact they do make falsifiable predictions. It is argued by Ioannidis that many cases of mutually exclusive theories being accepted as validated by research journals are caused by failure of the journals to take in empirical falsifications of non-normally distributed predictions, and not because mutually exclusive theories are true, which they cannot be, although two mutually exclusive theories can both be wrong and a third one correct.

### Generating values from normal distribution

In computer simulations, especially in applications of the Monte-Carlo method, it is often desirable to generate values that are normally distributed. The algorithms listed below all generate the standard normal deviates, since a N(μ, σ2) can be generated as X = μ + σZ, where Z is standard normal. All these algorithms rely on the availability of a random number generator U capable of producing uniform random variates.

The most straightforward method is based on the probability integral transform property: if U is distributed uniformly on (0,1), then Φ−1(U) will have the standard normal distribution. The drawback of this method is that it relies on calculation of the probit function Φ−1, which cannot be done analytically. Some approximate methods are described in Hart (1968) and in the erf article. Wichura gives a fast algorithm for computing this function to 16 decimal places, which is used by R to compute random variates of the normal distribution.

An easy-to-program approximate approach that relies on the central limit theorem is as follows: generate 12 uniform U(0,1) deviates, add them all up, and subtract 6 – the resulting random variable will have approximately standard normal distribution. In truth, the distribution will be Irwin–Hall, which is a 12-section eleventh-order polynomial approximation to the normal distribution. This random deviate will have a limited range of (−6, 6). Note that in a true normal distribution, only 0.00034% of all samples will fall outside ±6σ.

The Box–Muller method uses two independent random numbers U and V distributed uniformly on (0,1). Then the two random variables X and Y X=−2ln⁡Ucos⁡(2πV),Y=−2ln⁡Usin⁡(2πV). will both have the standard normal distribution, and will be independent. This formulation arises because for a bivariate normal random vector (X, Y) the squared norm X2 + Y2 will have the chi-squared distribution with two degrees of freedom, which is an easily generated exponential random variable corresponding to the quantity −2 ln(U) in these equations; and the angle is distributed uniformly around the circle, chosen by the random variable V.

The Marsaglia polar method is a modification of the Box–Muller method which does not require computation of the sine and cosine functions. In this method, U and V are drawn from the uniform (−1,1) distribution, and then S = U2 + V2 is computed. If S is greater or equal to 1, then the method starts over, otherwise the two quantities X=U−2ln⁡SS,Y=V−2ln⁡SS are returned. Again, X and Y are independent, standard normal random variables.

The Ratio method is a rejection method. The algorithm proceeds as follows:

Generate two independent uniform deviates U and V;

Compute X = √8/e (V − 0.5)/U;

Optional: if X2 ≤ 5 − 4e1/4U then accept X and terminate algorithm;

Optional: if X2 ≥ 4e−1.35/U + 1.4 then reject X and start over from step 1;

If X2 ≤ −4 ln U then accept X, otherwise start over the algorithm.

The two optional steps allow the evaluation of the logarithm in the last step to be avoided in most cases. These steps can be greatly improved so that the logarithm is rarely evaluated.

The ziggurat algorithm is faster than the Box–Muller transform and still exact. In about 97% of all cases it uses only two random numbers, one random integer and one random uniform, one multiplication and an if-test. Only in 3% of the cases, where the combination of those two falls outside the "core of the ziggurat" (a kind of rejection sampling using logarithms), do exponentials and more uniform random numbers have to be employed.

Integer arithmetic can be used to sample from the standard normal distribution. This method is exact in the sense that it satisfies the conditions of ideal approximation; i.e., it is equivalent to sampling a real number from the standard normal distribution and rounding this to the nearest representable floating point number.

There is also some investigation into the connection between the fast Hadamard transform and the normal distribution, since the transform employs just addition and subtraction and by the central limit theorem random numbers from almost any distribution will be transformed into the normal distribution. In this regard a series of Hadamard transforms can be combined with random permutations to turn arbitrary data sets into a normally distributed data.

### Numerical approximations for the normal cumulative distribution function and normal quantile function

The standard normal cumulative distribution function is widely used in scientific and statistical computing.

The values Φ(x) may be approximated very accurately by a variety of methods, such as numerical integration, Taylor series, asymptotic series and continued fractions. Different approximations are used depending on the desired level of accuracy.

Zelen & Severo (1964) give the approximation for Φ(x) for x > 0 with the absolute error |ε(x)| < 7.5·10−8 (algorithm 26.2.17): Φ(x)=1−φ(x)(b1t+b2t2+b3t3+b4t4+b5t5)+ε(x),t=11+b0x, where ϕ(x) is the standard normal probability density function, and b0 = 0.2316419, b1 = 0.319381530, b2 = −0.356563782, b3 = 1.781477937, b4 = −1.821255978, b5 = 1.330274429.

Hart (1968) lists dozens of approximations by means of rational functions, with or without exponentials, for the erfc() function, where erfc(x) = 1 - erf(x). His algorithms vary in the degree of complexity and the resulting precision, with a maximum absolute precision of 24 digits. An algorithm by West (2009) combines Hart's algorithm 5666 with a continued fraction approximation in the tail to provide a fast computation algorithm with 16-digit precision.

Cody (1969), after recalling the Hart68 solution is not suited for erf, gave a solution for both erf and erfc, with maximal relative error bound, via Rational Chebyshev Approximation.

Marsaglia (2004) suggested a simple algorithm based on the Taylor series expansion Φ(x)=12+φ(x)(x+x33+x53⋅5+x73⋅5⋅7+x93⋅5⋅7⋅9+⋯) for calculating Φ(x) with arbitrary precision. The drawback of this algorithm is comparatively slow calculation time (for example it takes over 300 iterations to calculate the function with 16 digits of precision when x = 10).

The GNU Scientific Library calculates values of the standard normal cumulative distribution function using Hart's algorithms and approximations with Chebyshev polynomials.

Dia (2023) proposes the following approximation of 1−Φ with a maximum relative error less than 2−53 (≈1.1×10−16) in absolute value: for x≥01−Φ(x)=(0.39894228040143268x+2.92678600515804815)(x2+8.42742300458043240x+18.38871225773938487x2+5.81582518933527391x+8.97280659046817350)(x2+7.30756258553673541x+18.25323235347346525x2+5.70347935898051437x+10.27157061171363079)(x2+5.66479518878470765x+18.61193318971775795x2+5.51862483025707963x+12.72323261907760928)(x2+4.91396098895240075x+24.14804072812762821x2+5.26184239579604207x+16.88639562007936908)(x2+3.83362947800146179x+11.61511226260603247x2+4.92081346632882033x+24.12333774572479110)e−x22 and for x<0,

1−Φ(x)=1−(1−Φ(−x))

Shore (1982) introduced simple approximations that may be incorporated in stochastic optimization models of engineering and operations research, like reliability engineering and inventory analysis. Denoting p = Φ(z), the simplest approximation for the quantile function is:

z=Φ−1(p)=5.5556[1−(1−pp)0.1186],p≥1/2

This approximation delivers for z a maximum absolute error of 0.026 (for 0.5 ≤ p ≤ 0.9999, corresponding to 0 ≤ z ≤ 3.719). For p < 1/2 replace p by 1 − p and change sign. Another approximation, somewhat less accurate, is the single-parameter approximation:

z=−0.4115{1−pp+log⁡[1−pp]−1},p≥1/2

The latter had served to derive a simple approximation for the loss integral of the normal distribution, defined by

L(z)=∫z∞(u−z)φ(u)du=∫z∞[1−Φ(u)]duL(z)≈{0.4115(p1−p)−z,p<1/2,0.4115(1−pp),p≥1/2.or, equivalently,L(z)≈{0.4115{1−log⁡[p1−p]},p<1/2,0.41151−pp,p≥1/2.

This approximation is particularly accurate for the right far-tail (maximum error of 10−3 for z ≥ 1.4). Highly accurate approximations for the cumulative distribution function, based on Response Modeling Methodology (RMM, Shore, 2011, 2012), are shown in Shore (2005).

Some more approximations can be found at: Error function#Approximation with elementary functions. In particular, small relative error on the whole domain for the cumulative distribution function ⁠Φ⁠ and the quantile function Φ−1 as well, is achieved via an explicitly invertible formula by Sergei Winitzki in 2008.

### Development

Some authors attribute the discovery of the normal distribution to de Moivre, who in 1738 published in the second edition of his The Doctrine of Chances the study of the coefficients in the binomial expansion of (a + b)n. De Moivre proved that the middle term in this expansion has the approximate magnitude of 2n/2πn, and that "If m or ⁠1/2⁠n be a Quantity infinitely great, then the Logarithm of the Ratio, which a Term distant from the middle by the Interval ℓ, has to the middle Term, is −2ℓℓn." Although this theorem can be interpreted as the first obscure expression for the normal probability law, Stigler points out that de Moivre himself did not interpret his results as anything more than the approximate rule for the binomial coefficients, and in particular de Moivre lacked the concept of the probability density function.

In 1823 Gauss published his monograph Theoria combinationis observationum erroribus minimis obnoxiae (transl. Theory of the Combination of Observations Least Subject to Errors) where among other things he introduces several important statistical concepts, such as the method of least squares, the method of maximum likelihood, and the normal distribution. Gauss used M, M′, M″, ... to denote the measurements of some unknown quantity V, and sought the most probable estimator of that quantity: the one that maximizes the probability φ(M − V) · φ(M′ − V) · φ(M″ − V) · ... of obtaining the observed experimental results. In his notation φΔ is the probability density function of the measurement errors of magnitude Δ. Not knowing what the function φ is, Gauss requires that his method should reduce to the well-known answer: the arithmetic mean of the measured values. Starting from these principles, Gauss demonstrates that the only law that rationalizes the choice of arithmetic mean as an estimator of the location parameter, is the normal law of errors:

φ𝛥=h√πe−hhΔΔ,

where h is "the measure of the precision of the observations". Using this normal law as a generic model for errors in the experiments, Gauss formulates what is now known as the non-linear weighted least squares method.

Although Gauss was the first to suggest the normal distribution law, Laplace made significant contributions. It was Laplace who first posed the problem of aggregating several observations in 1774, although his own solution led to the Laplacian distribution. It was Laplace who first calculated the value of the integral ∫ e−t2 dt = √π in 1782, providing the normalization constant for the normal distribution. For this accomplishment, Gauss acknowledged the priority of Laplace. Finally, it was Laplace who in 1810 proved and presented to the academy the fundamental central limit theorem, which emphasized the theoretical importance of the normal distribution.

It is of interest to note that in 1809 an Irish-American mathematician Robert Adrain published two insightful but flawed derivations of the normal probability law, simultaneously and independently from Gauss. His works remained largely unnoticed by the scientific community, until in 1871 they were exhumed by Abbe.

In the middle of the 19th century Maxwell demonstrated that the normal distribution is not just a convenient mathematical tool, but may also occur in natural phenomena: The number of particles whose velocity, resolved in a certain direction, lies between x and x + dx is

N⁡1απe−x2α2dx

### Naming

Today, the concept is usually known in English as the normal distribution or Gaussian distribution.  Other less common names include Gauss distribution, Laplace–Gauss distribution, the law of error, the law of facility of errors, Laplace's second law, and Gaussian law.

Gauss himself apparently coined the term with reference to the "normal equations" involved in its applications, with normal having its technical meaning of orthogonal rather than usual. However, by the end of the 19th century some authors had started using the name normal distribution, where the word "normal" was used as an adjective – the term now being seen as a reflection of this distribution being seen as typical, common – and thus normal. Peirce (one of those authors) once defined "normal" thus: "... the 'normal' is not the average (or any other kind of mean) of what actually occurs, but of what would, in the long run, occur under certain circumstances." Around the turn of the 20th century Pearson popularized the term normal as a designation for this distribution.

Many years ago I called the Laplace–Gaussian curve the normal curve, which name, while it avoids an international question of priority, has the disadvantage of leading people to believe that all other distributions of frequency are in one sense or another 'abnormal'.

Also, it was Pearson who first wrote the distribution in terms of the standard deviation σ as in modern notation. Soon after this, in year 1915, Fisher added the location parameter to the formula for normal distribution, expressing it in the way it is written nowadays:

df=12σ2πe−(x−m)2/(2σ2)dx.

The term standard normal distribution, which denotes the normal distribution with zero mean and unit variance came into general use around the 1950s, appearing in the popular textbooks by P. G. Hoel (1947) Introduction to Mathematical Statistics and Alexander M. Mood (1950) Introduction to the Theory of Statistics.

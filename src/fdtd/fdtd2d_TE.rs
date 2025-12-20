/// Параметры FDTD-модели
#[derive(Clone, Debug)]
pub struct FdtParams {
    pub d: f64,
    pub max_time_sec: f64,
    pub size_x_m: f64,
    pub size_y_m: f64,
    pub port_x_m: f64,
    pub port_y_m: f64,
    pub gauss_width_sec: f64,
    pub gauss_delay_factor: f64,
}

impl Default for FdtParams {
    fn default() -> Self {
        Self {
            d: 1e-3,
            max_time_sec: 1.1e-9,
            size_x_m: 0.3,
            size_y_m: 0.2,
            port_x_m: 0.15,
            port_y_m: 0.1,
            gauss_width_sec: 2e-11,
            gauss_delay_factor: 2.5,
        }
    }
}

#[derive(Debug)]
pub struct Fdtd2dTe {
    params: FdtParams,
    size_x: usize,
    size_y: usize,
    max_time: usize,
    dt: f64,
    gauss_width: f64,
    gauss_delay: f64,
    port_x: usize,
    port_y: usize,
    step: usize,
    ex: Vec<f64>,
    ey: Vec<f64>,
    hz: Vec<f64>,
    cexe: f64,
    cexh: f64,
    ceye: f64,
    ceyh: f64,
    chzh: f64,
    chze: f64,
}

impl Clone for Fdtd2dTe {
    fn clone(&self) -> Self {
        Self {
            params: self.params.clone(),
            size_x: self.size_x,
            size_y: self.size_y,
            max_time: self.max_time,
            dt: self.dt,
            gauss_width: self.gauss_width,
            gauss_delay: self.gauss_delay,
            port_x: self.port_x,
            port_y: self.port_y,
            step: self.step,
            ex: self.ex.clone(),
            ey: self.ey.clone(),
            hz: self.hz.clone(),
            cexe: self.cexe,
            cexh: self.cexh,
            ceye: self.ceye,
            ceyh: self.ceyh,
            chzh: self.chzh,
            chze: self.chze,
        }
    }
}

impl Fdtd2dTe {
    pub fn new(params: FdtParams) -> Self {
        // Физические константы
        let mu0 = std::f64::consts::PI * 4e-7;
        let eps0 = 8.854_187_817e-12_f64;
        let c = 1.0 / (mu0 * eps0).sqrt();

        // Расчет дискретных параметров
        let cdtds = 1.0 / 2.0_f64.sqrt();
        let dt = params.d / c * cdtds;
        let max_time = (params.max_time_sec / dt).ceil() as usize;

        let size_x = (params.size_x_m / params.d).ceil() as usize;
        let size_y = (params.size_y_m / params.d).ceil() as usize;
        let port_x = (params.port_x_m / params.d).ceil() as usize;
        let port_y = (params.port_y_m / params.d).ceil() as usize;

        let gauss_width = params.gauss_width_sec / dt;
        let gauss_delay = params.gauss_delay_factor * gauss_width;

        let total = size_x * size_y;
        let ex = vec![0.0; total];
        let ey = vec![0.0; total];
        let hz = vec![0.0; total];

        // Коэффициенты схемы (среда — вакуум, проводимость 0)
        let loss = 0.0;
        let loss_m = 0.0;
        let chzh = (1.0 - loss_m) / (1.0 + loss_m);
        let chze = 1.0 / (1.0 + loss_m) * (dt / (mu0 * params.d));
        let cexe = (1.0 - loss) / (1.0 + loss);
        let cexh = 1.0 / (1.0 + loss) * (dt / (eps0 * params.d));
        let ceye = cexe;
        let ceyh = cexh;

        Self {
            params,
            size_x,
            size_y,
            max_time,
            dt,
            gauss_width,
            gauss_delay,
            port_x,
            port_y,
            step: 1, // python начинается с q=1
            ex,
            ey,
            hz,
            cexe,
            cexh,
            ceye,
            ceyh,
            chzh,
            chze,
        }
    }

    #[inline(always)]
    fn idx(&self, x: usize, y: usize) -> usize {
        x * self.size_y + y
    }

    /// Выполняет один временной шаг. Возвращает true, если достигнут max_time.
    pub fn step(&mut self) -> bool {
        if self.step >= self.max_time {
            return true;
        }

        let q = self.step as f64;
        let sx = self.size_x;
        let sy = self.size_y;

        // Hz[:-1, :-1]
        for x in 0..(sx - 1) {
            for y in 0..(sy - 1) {
                let idx = self.idx(x, y);
                let hz_new = self.chzh * self.hz[idx]
                    + self.chze
                        * (self.ex[self.idx(x, y + 1)] - self.ex[idx]
                            - (self.ey[self.idx(x + 1, y)] - self.ey[idx]));
                self.hz[idx] = hz_new;
            }
        }

        // Ex[:-1, 1:-1]
        for x in 0..(sx - 1) {
            for y in 1..(sy - 1) {
                let idx = self.idx(x, y);
                let ex_new = self.cexe * self.ex[idx]
                    + self.cexh * (self.hz[self.idx(x, y)] - self.hz[self.idx(x, y - 1)]);
                self.ex[idx] = ex_new;
            }
        }

        // Ey[1:-1, :-1]
        for x in 1..(sx - 1) {
            for y in 0..(sy - 1) {
                let idx = self.idx(x, y);
                let ey_new = self.ceye * self.ey[idx]
                    - self.ceyh * (self.hz[self.idx(x, y)] - self.hz[self.idx(x - 1, y)]);
                self.ey[idx] = ey_new;
            }
        }

        // Источник
        if self.port_x < sx && self.port_y < sy {
            let src = (-(q - self.gauss_delay).powi(2) / self.gauss_width.powi(2)).exp();
            let idx = self.idx(self.port_x, self.port_y);
            self.hz[idx] += src;
        }

        self.step += 1;
        self.step >= self.max_time
    }

    pub fn reset(&mut self) {
        let params = self.params.clone();
        *self = Fdtd2dTe::new(params);
    }

    pub fn ey(&self) -> &[f64] {
        &self.ey
    }

    pub fn size(&self) -> (usize, usize) {
        (self.size_x, self.size_y)
    }

    pub fn step_index(&self) -> usize {
        self.step
    }
}

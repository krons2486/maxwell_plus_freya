/// Двумерный FDTD. Поляризация TMz. Граничные условия — PEC.
/// Алгоритм реализован по аналогии с Python‑версией и `Fdtd2dTe`.
use super::FdtParams;

#[derive(Debug)]
pub struct Fdtd2dTm {
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
    hx: Vec<f64>,
    hy: Vec<f64>,
    ez: Vec<f64>,
    chxh: f64,
    chxe: f64,
    chyh: f64,
    chye: f64,
    ceze: f64,
    cezh: f64,
}

impl Clone for Fdtd2dTm {
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
            hx: self.hx.clone(),
            hy: self.hy.clone(),
            ez: self.ez.clone(),
            chxh: self.chxh,
            chxe: self.chxe,
            chyh: self.chyh,
            chye: self.chye,
            ceze: self.ceze,
            cezh: self.cezh,
        }
    }
}

impl Fdtd2dTm {
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
        let hx = vec![0.0; total];
        let hy = vec![0.0; total];
        let ez = vec![0.0; total];

        // Параметры среды: вакуум, проводимость 0
        let loss = 0.0;
        let loss_m = 0.0;

        // Коэффициенты конечно‑разностной схемы,
        // сведенные к скалярам (eps = mu = 1, sigma = sigma_m = 0)
        let chxh = (1.0 - loss_m) / (1.0 + loss_m);
        let chxe = 1.0 / (1.0 + loss_m) * (dt / (mu0 * params.d));
        let chyh = chxh;
        let chye = chxe;
        let ceze = (1.0 - loss) / (1.0 + loss);
        let cezh = 1.0 / (1.0 + loss) * (dt / (eps0 * params.d));

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
            step: 1, // Python‑код начинает с q = 1
            hx,
            hy,
            ez,
            chxh,
            chxe,
            chyh,
            chye,
            ceze,
            cezh,
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

        // Hx[:, :-1]
        for x in 0..sx {
            for y in 0..(sy - 1) {
                let idx = self.idx(x, y);
                let hx_new = self.chxh * self.hx[idx]
                    - self.chxe * (self.ez[self.idx(x, y + 1)] - self.ez[idx]);
                self.hx[idx] = hx_new;
            }
        }

        // Hy[:-1, :]
        for x in 0..(sx - 1) {
            for y in 0..sy {
                let idx = self.idx(x, y);
                let hy_new = self.chyh * self.hy[idx]
                    + self.chye * (self.ez[self.idx(x + 1, y)] - self.ez[idx]);
                self.hy[idx] = hy_new;
            }
        }

        // Ez[1:-1, 1:-1]
        for x in 1..(sx - 1) {
            for y in 1..(sy - 1) {
                let idx = self.idx(x, y);
                let ez_new = self.ceze * self.ez[idx]
                    + self.cezh
                        * ((self.hy[idx] - self.hy[self.idx(x - 1, y)])
                            - (self.hx[idx] - self.hx[self.idx(x, y - 1)]));
                self.ez[idx] = ez_new;
            }
        }

        // Источник
        if self.port_x < sx && self.port_y < sy {
            let src = (-(q - self.gauss_delay).powi(2) / self.gauss_width.powi(2)).exp();
            let idx = self.idx(self.port_x, self.port_y);
            self.ez[idx] += src;
        }

        self.step += 1;
        self.step >= self.max_time
    }

    pub fn reset(&mut self) {
        let params = self.params.clone();
        *self = Fdtd2dTm::new(params);
    }

    pub fn ez(&self) -> &[f64] {
        &self.ez
    }

    pub fn size(&self) -> (usize, usize) {
        (self.size_x, self.size_y)
    }

    pub fn step_index(&self) -> usize {
        self.step
    }
}


